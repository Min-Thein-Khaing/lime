---
title: "ပုံဆွဲရန် Tools"
description: "Brush coverage၊ Smoothing၊ Clone Stamp၊ Alpha Lock နှင့် Tools Reset လုပ်နည်း။"
translationKey: reference/painting
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Tools ရွေးရန် {#tools}

| Tools | လုပ်ဆောင်ပုံ |
| --- | --- |
| Pencil | Foreground Color ဖြင့် Hard edge ရှိသော Stroke ဆွဲသည်။ Smoothing မသုံးပါ။ |
| Brush | Hardness နှင့် Smoothing ပြင်နိုင်သော Stroke ဆွဲသည်။ |
| Eraser | Alpha ဖယ်သည်။ Background swatch မှ Color ဖြင့် အစားထိုးခြင်း မဟုတ်ပါ။ |
| Blur Brush / Sharpen Brush | Stroke စချိန် Pixel များကို သုံးပြီး နေရာကွက်၍ Blur/Sharpen လုပ်သည်။ |
| Clone Stamp | Alt-click ဖြင့် ရွေးထားသော Source မှ ကူးဆွဲသည်။ `Shift+C` ဖြင့် ရွေးနိုင်သည်။ |
| Eyedropper | မြင်ရသောပေါင်းစပ်ပုံမှ Foreground Color ယူသည်။ Key `K` ဖြစ်သည်။ |

ပုံဆွဲခြင်းနှင့် Retouch လုပ်ခြင်းက Canvas၊ Active Layer နှင့် Selection အတွင်းသာ သက်ရောက်သည်။ Canvas အပြင်မှ စပြီး ဖိထားစဉ် အတွင်းဝင်ဆွဲနိုင်သည်။ Eyedropper နှင့် Clone source ရွေးခြင်းအတွက် Canvas အတွင်း Click လုပ်ရသည်။

## Opacity၊ Flow နှင့် Smoothing {#settings}

Size သည် ပုံ၏ Pixel ယူနစ်ဖြစ်သည်။ `[` နှင့် `]` က Zoom နှင့်မဆိုင်ဘဲ Size ပြောင်းသည်။ ပါဝင်သော Tool တွင် Hardness က Soft edge/Hard edge ကို ထိန်းသည်။ Opacity က Stroke တစ်ခု၏ ပြောင်းလဲမှုပမာဏကို ကန့်သတ်သည်။ Blur/Sharpen တွင် Strength ဟု ခေါ်သည်။

Buildup ကို မူလပိတ်ထားသည်။ ဖွင့်ထားလျှင် Flow က Dab coverage ကို Stroke ၏ Opacity limit အထိ တဖြည်းဖြည်းတိုးသည်။ Flow နည်းလျှင် Stroke တစ်ခုအတွင်း ထပ်ဆွဲရန် လိုနိုင်သည်။ ပိုသက်ရောက်လိုလျှင် Pointer လွှတ်ပြီး Stroke အသစ်စပါ။ Opacity သုည၊ သို့မဟုတ် Buildup ဖွင့်ထားစဉ် Flow သုညဆို ဘာမှမပြောင်းပါ။

Brush၊ Eraser၊ Blur၊ Sharpen နှင့် Clone Stamp တစ်ခုစီက **Off**၊ **Light**၊ **Strong** Smoothing Settings ကို သီးခြားမှတ်ထားသည်။ Smoothing သုံးလျှင် Pointer နောက် အနည်းငယ်လိုက်ပြီး Pointer ရပ်သောအခါ အမီလိုက်သည်။ Pointer လွှတ်လျှင် အဆုံးမှတ်အထိ ဆွဲပေးသည်။ Pencil နှင့် Path/Selection Outline Stroke များတွင် Smoothing မသုံးပါ။

## Clone Source Settings {#clone}

Alt-click က Source သတ်မှတ်သည်။ Sample All Layers က Stroke စချိန် မြင်ရသောပေါင်းစပ်ပုံ Snapshot ကို သုံးသည်။ Destination က Active visible Raster Layer ဖြစ်နေသည်။ Aligned က Stroke များကြား Offset ထိန်းသည်။ ပိတ်ထားလျှင် Stroke တိုင်း ရွေးထားသော Source မှ ပြန်စသည်။ Show Source က နမူနာယူနေသောနေရာနှင့် Brush footprint ပြသည်။ Sampling ပြောင်းလျှင် Source ရှင်းသည်။ Tab/Layer သို့မဟုတ် နေရာနှင့် အရွယ်အစားပြောင်းလျှင် Source ပြန်ရွေးပါ။

## Lock နှင့် Reset {#reset}

Alpha Lock က ရှိပြီးသား Alpha ကို ထိန်းသည်။ ရှိပြီးသား Pixel များ၏ Color ကို ပြောင်းနိုင်သော်လည်း Eraser/Cut/Clear ဖြင့် မဖျက်နိုင်ပါ။ Editable Text ပေါ် ပုံဆွဲရန် **Layer → Rasterize Text Layer** လုပ်ရသည်။

**Tools → Reset Current Tool** က လက်ရှိ Tool ၏ မူလ Settings ကို ပြန်ထားသည်။ **Reset All Tool Settings…** က Tools အားလုံး Reset မလုပ်မီ အတည်ပြုချက်မေးသည်။ Artwork၊ Swatch နှင့် Saved Gradient Preset များကို မပြောင်းပါ။

## ဆက်ဖတ်ရန် {#related}

- [ပုံဆွဲပြီး ပြန်ပြင်ရန်](../../guides/paint-retouch/)
- [Color နှင့် Gradient ဖြည့်ရန်](../../guides/fills-gradients/)
- [ပုံဆွဲခြင်း သို့မဟုတ် Fill လုပ်သော်လည်း မပြောင်းခြင်း](../../troubleshooting/painting/)
