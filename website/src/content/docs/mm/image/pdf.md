---
title: "PDF Import နှင့် Export"
description: "PDF pages နှင့် Raster resolution ရွေးပြီး Import လုပ်ရန်၊ Canvas ကို PDF တစ်မျက်နှာအဖြစ် Export လုပ်ရန်။"
translationKey: "image/pdf"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## ရွေးထားသော PDF pages ကို Import လုပ်ရန် {#import}

1. **File → Open…** မှ PDF ကို ရွေးပါ။ Encrypted PDF ကို မပံ့ပိုးပါ။
2. Page thumbnails ကို ရွေးပါ။ **All**၊ **None** သို့မဟုတ် `1, 3-5` ကဲ့သို့ Page numbers နှင့် Ranges ကို သုံးနိုင်သည်။ **Previous/Next** ဖြင့် Thumbnail အုပ်စုများ ကြည့်ပါ။ **Selected** က ရွေးထားသော Page ကို မြင်ကွင်းထဲ ယူပေးသည်။
3. **1–1200 DPI** အတွင်း Resolution ရွေးပါ။ DPI မြင့်လျှင် Pixels နှင့် Memory ပိုလိုသည်။ ရလာမည့် Canvas dimensions ကို Dialog တွင် ပြသည်။
4. **White** သို့မဟုတ် **Transparent** နောက်ခံ ရွေးပါ။ Transparent က မခြယ်ထားသော နေရာနှင့် မူလ Opacity ကို ထိန်းသည်။ ပုံထဲတွင် ရှိပြီးသား အဖြူရောင်ကို မဖယ်ပါ။
5. Document တစ်ခုထဲ Layers အဖြစ် သို့မဟုတ် သီးခြား Tabs အဖြစ် ရွေးပါ။ Layers အတွက် ပထမရွေးထားသော Page ၏ Canvas သို့မဟုတ် **Fit All Pages** ကို ရွေးပါ။ Fit All Pages က ရွေးထားသမျှထဲမှ အကြီးဆုံး Width နှင့် Height ကို သုံးသည်။ Page တစ်ခုစီကို အရွယ်ညီအောင် Scale မလုပ်ဘဲ Center ချသည်။
6. Size သတိပေးချက်များ စစ်ပြီး **Import** ကို ရွေးပါ။

**ရလဒ်:** ရွေးထားသော Pages က Raster layers သို့မဟုတ် Image tabs ဖြစ်လာမည်။ မသိမ်းရသေးသော Projects ဖြစ်ပြီး Editable PDF objects မဟုတ်ပါ။ ဆက်ပြင်ရန် `.lime` ဖြင့် Save လုပ်ပါ။

**File → Open as Layers…** က PDF တစ်ဖိုင်ကို ရှိပြီးသား Canvas ထဲသို့ ထည့်သည်။ Canvas အရွယ် မပြောင်းပါ။ Import သုံးမရလျှင် ပြထားသော အကြောင်းပြချက်ကို ဖတ်ပြီး Page ရွေးထားမှု၊ DPI သို့မဟုတ် Size limits ကို ပြင်ပါ။ Cancel က Import ရလဒ်ကို ပယ်သည်။ လုပ်နေဆဲ Page operation ပြီးမှ Cancel အဆုံးသတ်နိုင်သည်။

## လက်ရှိ Canvas ကို Export လုပ်ရန် {#export}

1. Text edit ကို အဆုံးသတ်ပြီး အခြား Canvas previews ကို Apply/Cancel လုပ်ပါ။
2. **File → Export…** မှ **PDF** ကို ရွေးပါ။
3. DPI သတ်မှတ်ပြီး ပြထားသော Physical page size ကို စစ်ပါ။ DPI က Canvas ကို Resample မလုပ်ဘဲ စာမျက်နှာအရွယ်ကို ပြောင်းသည်။ Canvas ကြီးလျှင် PDF page limits အတွင်း ဝင်ရန် Minimum DPI ပိုမြင့်နိုင်သည်။
4. ဆက်သွားပြီး သီးခြား Destination ရွေးပါ။

ရလဒ်က လက်ရှိ Canvas တစ်ခုလုံးပါသော **Flattened page တစ်မျက်နှာ** ဖြစ်သည်။ အဖြူနောက်ခံပေါ်ရှိ Lossless RGB image ဖြစ်သည်။ Text ကို Search မလုပ်နိုင်ပါ။ Paths က Vector output မဟုတ်ပါ။ Transparent နေရာများ အဖြူဖြစ်သည်။ Document tabs များကို Multipage PDF အဖြစ် မပေါင်းပေးပါ။

## ဆက်ဖတ်ရန် {#related}

- [ပုံများကို Layers အဖြစ် Import လုပ်ရန်](../../image/import-layers/)
- [Project Save နှင့် ပုံ Export](../../reference/files/)
- [File၊ Text နှင့် Recovery ပြဿနာများ](../../troubleshooting/files-text-recovery/)
