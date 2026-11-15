#!/usr/bin/env python3
"""Exercise release signing failure paths without keys or Apple services."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

SCRIPTS = Path(__file__).resolve().parent
MOCK = '''#!/usr/bin/env python3
import json, os, pathlib, sys
name = pathlib.Path(sys.argv[0]).name
args = sys.argv[1:]
with open(os.environ['MOCK_TRACE'], 'a') as trace:
    trace.write(json.dumps([name, *args]) + '\\n')
if name == 'uname':
    print('Darwin')
elif name == 'codesign':
    if '--force' in args and os.environ.get('MOCK_SIGN_FAIL'):
        sys.exit(1)
    if '-dv' in args:
        print('Authority=Developer ID Application: Test (TEAM)', file=sys.stderr)
elif name == 'ditto':
    pathlib.Path(args[-1]).touch()
elif name == 'xcrun' and args[:2] == ['notarytool', 'submit']:
    if os.environ.get('MOCK_EMPTY_SUBMIT'):
        print('HTTP 403: required agreement missing', file=sys.stderr)
        sys.exit(1)
    print(json.dumps({'id': 'test-submission', 'status': os.environ.get('MOCK_STATUS', 'Accepted')}))
    sys.exit(int(os.environ.get('MOCK_SUBMIT_EXIT', '0')))
elif name == 'xcrun' and args[:2] == ['notarytool', 'log']:
    pathlib.Path(args[-1]).write_text('{}')
'''


class SigningTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='lime-signing-test-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.bin = self.root / 'bin'
        self.bin.mkdir()
        for name in ('uname', 'codesign', 'ditto', 'xcrun', 'spctl'):
            executable = self.bin / name
            executable.write_text(MOCK)
            executable.chmod(0o755)
        self.trace = self.root / 'trace.jsonl'
        self.app = self.root / 'Lime.app'
        self.app.mkdir()
        self.env = {key: value for key, value in os.environ.items()
                    if not key.startswith(('APPLE_', 'MOCK_'))}
        self.env.update(PATH=f'{self.bin}{os.pathsep}{os.environ["PATH"]}',
                        MOCK_TRACE=str(self.trace),
                        APPLE_SIGNING_IDENTITY='Developer ID Application: Test (TEAM)',
                        APPLE_NOTARY_PROFILE='test-profile')

    def run_script(self, script, artifact=None):
        result = subprocess.run(['sh', str(SCRIPTS / script), str(artifact or self.app)],
                                env=self.env, capture_output=True, text=True)
        calls = [json.loads(line) for line in self.trace.read_text().splitlines()]
        return result, calls

    def test_app_signing_uses_hardened_runtime_timestamp_and_keychain(self):
        self.env['APPLE_SIGNING_KEYCHAIN'] = '/test/signing.keychain-db'
        result, calls = self.run_script('sign-macos.sh')
        self.assertEqual(result.returncode, 0, result.stderr)
        sign = next(call for call in calls if '--force' in call)
        for flag in ('--timestamp', '--options', 'runtime', '--keychain', '/test/signing.keychain-db'):
            self.assertIn(flag, sign)

    def test_signing_failure_never_falls_back(self):
        self.env['MOCK_SIGN_FAIL'] = '1'
        result, calls = self.run_script('sign-macos.sh')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(sum('--force' in call for call in calls), 1)
        self.assertFalse(any('-' in call for call in calls))

    def test_accepted_app_is_stapled_validated_and_assessed(self):
        self.env['APPLE_NOTARY_KEYCHAIN'] = '/test/notary.keychain-db'
        result, calls = self.run_script('notarize-macos.sh')
        self.assertEqual(result.returncode, 0, result.stderr)
        submit = next(call for call in calls if call[:3] == ['xcrun', 'notarytool', 'submit'])
        self.assertIn('--keychain', submit)
        self.assertEqual([call[2] for call in calls if call[:2] == ['xcrun', 'stapler']],
                         ['staple', 'validate'])
        self.assertTrue(any(call[:4] == ['spctl', '--assess', '--type', 'execute'] for call in calls))
        self.assertTrue(self.app.with_suffix('.app.notarization-log.json').exists())

    def test_rejected_submission_never_staples(self):
        self.env['MOCK_STATUS'] = 'Invalid'
        result, calls = self.run_script('notarize-macos.sh')
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(any(call[:2] == ['xcrun', 'stapler'] for call in calls))
        self.assertTrue(any(call[:3] == ['xcrun', 'notarytool', 'log'] for call in calls))

    def test_failed_submit_never_staples_even_with_accepted_output(self):
        self.env['MOCK_SUBMIT_EXIT'] = '1'
        result, calls = self.run_script('notarize-macos.sh')
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(any(call[:2] == ['xcrun', 'stapler'] for call in calls))

    def test_dmg_is_assessed_as_a_distribution_container(self):
        dmg = self.root / 'Lime.dmg'
        dmg.touch()
        result, calls = self.run_script('notarize-macos.sh', dmg)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(any(call[:4] == ['spctl', '--assess', '--type', 'open'] and
                            'context:primary-signature' in call for call in calls))
        self.assertFalse(any(call[0] == 'ditto' for call in calls))

    def test_empty_service_error_keeps_original_failure_without_json_traceback(self):
        self.env['MOCK_EMPTY_SUBMIT'] = '1'
        result, calls = self.run_script('notarize-macos.sh')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('required agreement missing', result.stderr)
        self.assertNotIn('Traceback', result.stderr)
        self.assertFalse(any(call[:2] == ['xcrun', 'stapler'] for call in calls))

    def test_signed_packaging_requires_credentials_before_building(self):
        self.env['LIME_SIGNED_RELEASE'] = '1'
        self.env.pop('APPLE_NOTARY_PROFILE')
        result, calls = self.run_script('package-macos.sh', '--existing-app')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('APPLE_NOTARY_PROFILE', result.stderr)
        self.assertEqual(calls, [['uname', '-s']])


if __name__ == '__main__':
    unittest.main()
