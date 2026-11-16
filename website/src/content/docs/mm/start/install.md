---
title: "Lime ထည့်သွင်းရန်"
description: "Windows၊ Apple silicon Mac သို့မဟုတ် Linux အတွက် Lime ကို Download လုပ်ပြီး ဖွင့်ရန်။"
translationKey: "start/install"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Download ရွေးရန် {#download}

[Releases](https://github.com/eimg/lime/releases) မှ Lime ကို Download လုပ်ပါ။ သုံးနေသော Operating system နှင့် Processor အတွက် ဖိုင်ကို ရွေးပါ။

| ကွန်ပျူတာ | Download | လိုအပ်ချက် |
| --- | --- | --- |
| Windows x64 | `.exe` | Windows 10 နှင့်အထက်၊ DirectX၊ Microsoft Visual C++ 2015–2022 x64 runtime |
| Apple silicon Mac | `.dmg` | Apple M-series Mac။ Intel Mac တွင် သုံးမရပါ။ |
| Linux x86_64 | x86_64 `.tar.gz` | glibc 2.35 နှင့်အထက်၊ Vulkan၊ X11 သို့မဟုတ် Wayland နှင့် File chooser portal |
| Linux ARM64 | aarch64 `.tar.gz` | အထက်ပါ Linux libraries နှင့် ကိုက်ညီသော ARM64 Vulkan driver |

## Lime ဖွင့်ရန် {#steps}

1. **Windows:** EXE ကို Download လုပ်ပြီး ဖွင့်ပါ။ Installer မရှိပါ။ ဖိုင်များကို Lime နှင့် အလိုအလျောက် ချိတ်မပေးပါ။ EXE ကို ပြန်ရှာရလွယ်သော Folder တွင် ထားပါ။
2. **Mac:** DMG ကို ဖွင့်ပြီး **Lime** ကို **Applications** ထဲ ဆွဲထည့်ပါ။ ထိုနေရာမှ Lime ကို ဖွင့်ပါ။ macOS release သည် Developer ID Signed ဖြစ်ပြီး Apple Notarized လုပ်ထားသည်။
3. **Linux:** Archive ကို ဖြည်ပြီး ထို Folder မှ `./lime` ကို ဖွင့်ပါ။ ပါလာသော Desktop launcher ကို သုံးမည်ဆိုလျှင် ဖိုင်များကို အတူထားပါ။ အလိုအလျောက် Install မလုပ်ပါ။

**ရလဒ်:** ပုံဖွင့်ရန် သို့မဟုတ် ပုံအသစ်ဖန်တီးရန် အသင့်ဖြစ်သော Workspace ပေါ်လာမည်။

## Lime ဖွင့်မရလျှင် {#troubleshooting}

Download က စက်၏ Processor နှင့် ကိုက်ညီကြောင်း စစ်ပါ။ Windows build က Unsigned ဖြစ်သဖြင့် Unknown publisher သတိပေးချက် ပေါ်နိုင်သည်။ Linux တွင် Window အလွတ်ပေါ်လျှင် Vulkan driver ကို စစ်ပါ။ Open/Save dialog မပေါ်လျှင် Portal backend ကို စစ်ပါ။ လိုအပ်သော Package များကို [Linux runtime requirements](https://github.com/eimg/lime/blob/main/linux/README.txt) တွင် ကြည့်နိုင်သည်။

Update စစ်ရန် macOS တွင် **Lime → Check for Updates…**၊ Windows/Linux တွင် **Help → Check for Updates…** ကို သုံးပါ။ Version ကို Status bar တွင်လည်း ပြထားသည်။

## ဆက်ဖတ်ရန် {#related}

- [Workspace အစိတ်အပိုင်းများ](../../start/workspace/)
- [ပထမဆုံး ပုံပြင်ရန်](../../start/first-edit/)
