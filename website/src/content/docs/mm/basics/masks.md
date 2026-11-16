---
title: "Layer mask သုံးရန်"
description: "Selection မှ Layer mask ဖန်တီးပြီး Compare၊ Invert၊ Apply နှင့် Remove လုပ်နည်း။"
translationKey: "basics/masks"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Pixel မဖျက်ဘဲ ဖုံးရန် {#meaning}

Layer mask က Raster layer ၏ ဘယ်နေရာကို ဘယ်လောက် မြင်ရမည်ဆိုသည်ကို ထိန်းသည်။ Coverage အပြည့်ရှိသော နေရာကို မြင်ရပြီး Coverage သုညနေရာကို ဖုံးထားသည်။ Partial coverage က Soft edges ပေးသည်။ Mask က Layer pixels ၏ Transparency နှင့် Opacity တို့နှင့် ပေါင်းစပ်သက်ရောက်သည်။

Selection က နောက်လုပ်မည့် Edit ကို ကန့်သတ်သည်။ Layer mask က Layer တစ်ခုတွင် ဆက်ရှိနေပြီး ပြင်ခြင်း၊ Remove သို့မဟုတ် Apply မလုပ်မချင်း မြင်ရပုံကို ထိန်းသည်။ Mask ကို ထိန်းထားရန် `.lime` ဖြင့် Save လုပ်ပါ။

## Selection မှ Mask ဖန်တီးရန် {#steps}

1. **Layers** မှ မြင်ရသော Raster layer ကို ရွေးပါ။ Editable Text တွင် Layer mask သုံးမရပါ။ Rasterize လုပ်ရန် လိုလျှင် Editable duplicate ကို အရင်ထားပါ။
2. မြင်စေလိုသော နေရာကို Select လုပ်ပါ။ Soft edges လိုလျှင် အရင် Feather လုပ်ပါ။
3. **Layer → Mask → From Selection** ကို ရွေးပါ။ Selection coverage က Layer mask ဖြစ်လာမည်။ ရှိပြီးသား Mask ရှိလျှင် အစားထိုးမည်။
4. Edit လုပ်နိုင်သောနေရာကို ကန့်သတ်ထားရန် မလိုတော့လျှင် **Select → Select None** ကို ရွေးပါ။
5. Mask မဖျက်ဘဲ မူလပုံနှင့် နှိုင်းယှဉ်ရန် **Layer → Mask → Enable / Disable** ကို သုံးပါ။

**ရလဒ်:** ရွေးထားသော အပိုင်းကို မြင်ရမည်။ ဖုံးထားသော အပိုင်းတွင် အောက် Layer သို့မဟုတ် Checkerboard ကို မြင်ရမည်။ မူလ Pixel များကို ဆက်ထိန်းထားသည်။

## Mask ပြန်ပြင်ရန် သို့မဟုတ် အပြီးသတ်ရန် {#revise}

| Layer → Mask ရှိ Command | ရလဒ် |
| --- | --- |
| **Mask to Selection** | Mask coverage ကို Selection ထဲ ကူးပြီး ဆက်ပြင်နိုင်သည်။ |
| **Invert** | မြင်ရသော နေရာနှင့် ဖုံးထားသော နေရာကို ပြောင်းပြန်လုပ်သည်။ |
| **From Selection** | လက်ရှိ Selection ဖြင့် Mask ကို အစားထိုးသည်။ |
| **Apply** | Enabled mask ၏ Coverage ကို Pixel alpha ထဲ ပေါင်းပြီး သီးခြား Mask ကို ဖယ်သည်။ |
| **Remove** | မူလ Pixel မဖျက်ဘဲ Mask ကို ဖယ်သည်။ |

**Apply** သုံးရန် Mask ကို Enable လုပ်ထားပြီး **Alpha Lock** ပိတ်ထားရမည်။

Lime တွင် Mask ကို Selection နှင့် ဤ Menu commands မှ ပြင်သည်။ Layer mask ပေါ် Brush ဖြင့် တိုက်ရိုက် ပုံဆွဲရန် သီးခြား Target မရှိပါ။ Outline ပြင်ရန် **Mask to Selection** ဖြင့် ယူပြီး Selection ကို ပြင်ပါ။ ထို့နောက် **From Selection** ဖြင့် ပြန်အစားထိုးပါ။

## ဆက်ဖတ်ရန် {#related}

- [Selection၊ Antialias နှင့် Feather](../../basics/selections/)
- [နောက်ခံဖယ်ရန်](../../guides/remove-background/)
- [Project Save နှင့် ပုံ Export](../../reference/files/)
