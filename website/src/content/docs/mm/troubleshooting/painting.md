---
title: "ပုံဆွဲခြင်း သို့မဟုတ် Fill လုပ်သော်လည်း မပြောင်းခြင်း"
description: "Active Layer၊ Selection၊ Lock၊ Brush Settings နှင့် Clone Source စစ်နည်း။"
translationKey: troubleshooting/painting
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## ပုံဆွဲနေသော Layer စစ်ရန် {#target}

ပုံဆွဲမရလျှင် သို့မဟုတ် Brush အလုပ်မလုပ်လျှင် Artwork မပြောင်းမီ အောက်ပါ Settings များကို စစ်ပါ။

1. **Layers** တွင် ပြင်လိုသော Row ကို ရွေးပါ။ မြင်နေသောပုံနှင့် ပြင်နေသော Active Layer က မတူနိုင်ပါ။
2. Active Layer မြင်ရပြီး Opacity သုညမဟုတ်ကြောင်း စစ်ပါ။ အပေါ်ရှိ Layer က ဆွဲထားသော Pixel များကို ဖုံးနေနိုင်သည်။ စစ်ရန် အပေါ် Layer များကို ယာယီဖျောက်ပါ။
3. Editable Text ဖြစ်လျှင် Project copy Save လုပ်ပြီး Pixel Tools မသုံးမီ **Layer → Rasterize Text Layer** လုပ်ပါ။ စာလုံးပြင်လိုလျှင် Text ရွေးပါ။
4. **View → Toggle Layer Boundary** ရွေးပါ။ Layer ၏ Pixel စတုဂံအပြင်တွင် ပုံဆွဲလျှင် Layer ကို ချဲ့မပေးပါ။ Canvas အပြည့် Pixel ဧရိယာလိုလျှင် **Layer → Layer to Canvas Size** သုံးပါ။

## Selection နှင့် Mask စစ်ရန် {#selection}

**Select → Select None** ဖြင့် Selection ကန့်သတ်ချက် ဖယ်ပါ။ Stroke ငယ်တစ်ခု စမ်းဆွဲပြီး Undo လုပ်ပါ။ Selection သေးလွန်းခြင်း သို့မဟုတ် မျက်နှာပြင်အပြင်ရောက်နေခြင်းကြောင့် Pointer အောက်ရှိနေရာတွင် ပုံဆွဲမရနိုင်ပါ။

Enabled Layer Mask က ဆွဲထားသော Pixel အသစ်များကို ဖျောက်ထားနိုင်သည်။ **Layer → Mask → Enable / Disable** ဖြင့် နှိုင်းယှဉ်ပါ။ Mask နှင့် Source Pixel များကို မဖျက်ပါ။ ပုံဆွဲခြင်းက Raster ကိုသာ ပြင်ပြီး Mask ကို မပြင်ပါ။ Mask coverage ပြောင်းရန် Selection အခြေခံ Mask commands သုံးပါ။

## Alpha နှင့် Brush Settings စစ်ရန် {#settings}

**Layer → Toggle Alpha Lock** က ရှိပြီးသား Alpha ပြောင်းခွင့်ကို ထိန်းသည်။ ဖွင့်ထားလျှင် လုံးဝ Transparent ဖြစ်သော Pixel တွင် ပုံဆွဲမရသကဲ့သို့ Eraser/Cut/Clear ဖြင့်လည်း မဖျက်နိုင်ပါ။ Transparency ပြောင်းလိုလျှင် Alpha Lock ပိတ်ပါ။

Opacity၊ သို့မဟုတ် Blur/Sharpen ၏ Strength ကို စစ်ပါ။ Buildup ဖွင့်ထားလျှင် Flow လည်း သုညထက်များရမည်။ Flow နည်းပြီး Hardness နည်းလျှင် တစ်ကြိမ်ဆွဲရုံဖြင့် မသိသာနိုင်ပါ။ မြင်လွယ်သော Color ဖြင့် စမ်းပါ။ **Tools → Reset Current Tool** က Artwork နှင့် Color မပြောင်းဘဲ Default Settings ပြန်ထားသည်။

## Fill နှင့် Clone စစ်ရန် {#sampling}

Bucket ၏ Connected mode နှင့် Magic Wand တွင် Canvas အတွင်း Click လုပ်ရသည်။ Tolerance 0 က Color အတိအကျတူမှ ယူသည်။ အခြား Layer များက နယ်နိမိတ်ဖွဲ့ထားလျှင် Sample All Layers ဖွင့်ပါ။ Fill က Active Layer ပေါ်သာ သက်ရောက်သည်။ Selection mode က Matching controls မသုံးဘဲ Selection အတွင်း ဖြည့်သည်။

Clone Stamp တွင် Alt-click ဖြင့် ရွေးထားသော Source လိုသည်။ Tab/Layer၊ Geometry သို့မဟုတ် Sampling ပြောင်းပြီးလျှင် ပြန်ရွေးပါ။ Source က Transparent ဖြစ်နေလျှင် မြင်ရသော Stroke မပေါ်နိုင်ပါ။ **Show Source** ဖွင့်ပြီး နမူနာယူနေသောနေရာ စစ်ပါ။

## ဆက်ဖတ်ရန် {#related}

- [ပုံဆွဲရန် Tools](../../reference/painting/)
- [Layer mask သုံးရန်](../../basics/masks/)
- [Color နှင့် Gradient ဖြည့်ရန်](../../guides/fills-gradients/)
