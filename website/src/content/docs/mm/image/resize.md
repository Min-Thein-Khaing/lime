---
title: "Image သို့မဟုတ် Canvas အရွယ်ပြင်ရန်"
description: "Image တစ်ခုလုံး Scale လုပ်ခြင်း၊ Canvas bounds ပြောင်းခြင်းနှင့် Layer တစ်ခု Scale လုပ်ခြင်း ခွဲခြားရန်။"
translationKey: "image/resize"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## အရွယ်ပြင်နည်း ရွေးရန် {#choose}

ပုံ သို့မဟုတ် ဓာတ်ပုံ အရွယ်ပြင်ရန် Pixel အရွယ်အစား၊ Canvas နေရာ သို့မဟုတ် Layer တစ်ခုတည်းကို ပြောင်းမလား အရင်ရွေးပါ။

| လိုချင်သော ရလဒ် | Command | ပြောင်းသည့်အရာ |
| --- | --- | --- |
| ပုံတစ်ခုလုံး ပိုကြီး/ပိုသေးစေရန် | **Image → Scale Image…** | Document နှင့် Layers ကို Resample လုပ်သည်။ |
| Artwork ပတ်လည် နေရာတိုးရန် သို့မဟုတ် Canvas bounds ကျုံ့ရန် | **Image → Canvas Size…** | Artwork မဆန့်ဘဲ Canvas rectangle ကို ပြောင်းသည်။ |
| Raster layer တစ်ခု အရွယ်ပြင်ရန် | **Layer → Transform → Scale Layer** | Active layer ကို သီးခြား Preview လုပ်သည်။ |
| Layer ကို Canvas အတွင်း အချိုးမပျက် အံဝင်စေရန် | **Layer → Fit Layer to Canvas** | Layer တစ်ခုလုံး Scale လုပ်ပြီး Center ချသည်။ |
| Layer ကို Canvas အရွယ် Crop/Pad လုပ်ရန် | **Layer → Layer to Canvas Size** | Pixel နေရာ မရွှေ့ဘဲ Layer bounds ပြောင်းသည်။ |

## ပုံတစ်ခုလုံး Scale လုပ်ရန် {#scale}

**Scale Image…** သုံးရန် Hidden layers အပါအဝင် Editable Text layers အားလုံးကို Rasterize လုပ်ထားရမည်။ Editable `.lime` မူရင်းကို အရင် Save လုပ်ပါ။ ပြီးလျှင် Text layer တစ်ခုစီကို ရွေးပြီး **Layer → Rasterize Text Layer** လုပ်ပါ။ မူရင်း Project ကို ဆက်ပြင်နိုင်အောင် ထားလိုလျှင် Flattened copy ကို Export လုပ်ပြီး ထိုမိတ္တူကို ပြန်ဖွင့်ကာ Scale လုပ်နိုင်သည်။

1. **Image → Scale Image…** ကို ရွေးပါ။
2. လိုသော **Width** သို့မဟုတ် **Height** ကို Pixels ဖြင့် ရိုက်ပါ။ ပုံအချိုးမပျက်စေရန် **Lock aspect ratio** ကို ဖွင့်ထားပါ။
3. Resampling method ရွေးပြီး **Scale** ကို နှိပ်ပါ။
4. ရလဒ်ကို သင့်တော်သော Zoom ဖြင့် စစ်ပြီး Project Save လုပ်ပါ သို့မဟုတ် မိတ္တူ Export လုပ်ပါ။

**Nearest** က Hard pixel edges ကို ထိန်းသည်။ **Bilinear** က အနီးရှိ Pixels ကို ရောစပ်သည်။ **Bicubic** က သာမန် Artwork အတွက် ပိုချောမွေ့သော Resampling ပေးသည်။ ပုံချဲ့လျှင် Pixel အရေအတွက် တိုးသော်လည်း မူလပုံတွင် မရှိသော Detail ကို ပြန်မရနိုင်ပါ။

## Canvas နေရာတိုးရန် {#canvas}

1. **Image → Canvas Size…** ကို ရွေးပါ။
2. Dimensions အသစ်ကို ရိုက်ပါ။
3. ဘောင်အသစ်အတွင်း Artwork ထားမည့် နေရာကို Anchor ဖြင့် ရွေးပါ။ Center anchor က အရွယ်ပြောင်းမှုကို ပတ်လည် မျှပေးသည်။ Edge/Corner anchor က ရွေးထားသောဘက်ကို တည်ငြိမ်စွာ ထားသည်။
4. **Resize** ကို နှိပ်ပြီး Canvas edges ကို စစ်ပါ။

**ရလဒ်:** Document bounds ပြောင်းပြီး Artwork ၏ Pixel scale က မူလအတိုင်း ရှိမည်။ Editable Text ရှိလည်း **Canvas Size…** ကို သုံးနိုင်သည်။ Canvas အသစ်အပြင်ဘက် Layer pixels ကို ဆက်ထိန်းထားသည်။ Canvas ကျုံ့လျှင် ထို Pixels ကို ပေါင်းစပ်ပုံနှင့် Export ထဲတွင် မမြင်ရတော့သော်လည်း မဖျက်ပါ။ ပြန်ဖော်ရန် Canvas ကို ချဲ့ပါ သို့မဟုတ် Undo လုပ်ပါ။

## Canvas မပြောင်းဘဲ Layer ကို Fit လုပ်ရန် {#fit}

မြင်ရပြီး Position Lock မရှိသော Raster layer ကို ရွေးကာ **Layer → Fit Layer to Canvas** ကို သုံးပါ။ Transparent margins ပါ ထည့်တွက်ပြီး Layer သေးလျှင် ချဲ့နိုင်သည်။ **Scale Layer** သုံးနေစဉ် Fit က ထို Tool ၏ Preview ကို ပြောင်းပေးသဖြင့် **Apply** လုပ်ရန် လိုသေးသည်။ အခြား Tools မှ သုံးလျှင် ချက်ချင်း Commit လုပ်သည်။ Editable Text ကို Pixel scale မလုပ်မီ Rasterize လုပ်ရမည်။

## ဆက်ဖတ်ရန် {#related}

- [Layer transforms](../../reference/transforms/)
- [ပုံကို Crop လုပ်ရန်](../../image/crop/)
- [ပုံများကို Layers အဖြစ် Import လုပ်ရန်](../../image/import-layers/)
