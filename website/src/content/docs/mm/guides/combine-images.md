---
title: "ပုံများပေါင်းပြီး နေရာချရန်"
description: "ပုံများကို Layer အဖြစ် Import လုပ်၍ Scale၊ Move နှင့် Movement Link သုံးနည်း။"
translationKey: guides/combine-images
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## ပုံများပေါင်းရန် {#steps}

1. Canvas အရွယ်အစားအဖြစ် သုံးမည့်ပုံကို ဖွင့်ပါ။ အခြားပုံများ ထည့်ရန် **File → Open as Layers…** သုံးပါ။ ပုံအသစ်များကို Canvas အလယ်တွင် နေရာချပေးသည်။ ပုံဝင်ဆံ့အောင် Canvas ကို အလိုအလျောက် မချဲ့ပါ။
2. **Layers** တွင် Layer ရွေးပါ။ ရှေ့နောက်အစီအစဉ်ပြောင်းရန် **Layer → Raise Layer**၊ **Lower Layer** သုံးပါ၊ သို့မဟုတ် Layer Row ကို ဆွဲရွှေ့ပါ။
3. **Scale Layer** (`Shift+S`) ရွေးပါ။ Handle ဆွဲပါ၊ သို့မဟုတ် Aspect ratio ကို Lock ထားပြီး Width၊ Height ရိုက်ပါ။ Preview အဆင်ပြေလျှင် **Apply** လုပ်ပါ။
4. **Move** (`M`) ဖြင့် Layer ကို နေရာချပါ။ တိကျစွာရွှေ့ရန် X/Y ရိုက်ပြီး Return နှိပ်ပါ။ Arrow key က တစ်ကြိမ်လျှင် ပုံ၏ Pixel တစ်ခု၊ Shift နှင့်ဆို ဆယ်ခု ရွှေ့ပေးသည်။
5. Canvas နှင့် ညှိရန် Move ၏ Alignment ခလုတ်များ သို့မဟုတ် **Layer → Align** သုံးပါ။ ပုံများ ထပ်နေသည့်နေရာကို ဖျော့ပြရန် **Layer → Layer Opacity…** ပြင်ပါ။
6. **`.lime`** ဖြင့် Save လုပ်ပါ။ ပေါင်းထားသောပုံ သီးခြားထုတ်ရန် **File → Export…** သုံးပါ။

## ရလဒ် {#result}

Project ထဲတွင် ပုံတစ်ပုံစီကို သီးခြား Layer အဖြစ် ဆက်ပြင်နိုင်မည်။ Export တွင် Canvas အတွင်းရှိနေရာသာ ပါမည်။ Canvas အပြင်ရောက်နေသော Pixel များ မပါပါ။

## Layer များ အတူရွှေ့ရန် {#links}

အတူရွှေ့လိုသော Layer တစ်ခုစီ၏ Chain toggle ကို ဖွင့်ပါ။ Movement Link ဖွင့်ထားသော Layer တစ်ခုကို ရွှေ့လျှင် အခြားဖွင့်ထားသော Layer များလည်း အတူရွှေ့မည်။ ဖျောက်ထားသော Layer များလည်း ပါသည်။ Scale နှင့် Rotate က Layer တစ်ခုချင်းစီကိုသာ သက်ရောက်သည်။ ပါဝင်သော Layer တစ်ခုခုတွင် Position Lock ရှိလျှင် အားလုံးရွှေ့မရပါ။

Drop Shadow နှင့် Stroke Layer များက Attachment ကြောင့် Source နောက်လိုက်ရွှေ့သည်။ Source Pixel သို့မဟုတ် Text ပြောင်းသော်လည်း ထို Effect ပုံကို ပြန်မလုပ်ပါ။ သီးခြားရွှေ့ရန် **Layer → Detach from Source** သုံးပါ။ Raster Layer ကို ချက်ချင်းအပေါ်ရှိ Layer နှင့် တွဲရန် **Attach to Layer Above** သုံးနိုင်သည်။ တွဲလိုက်ရုံဖြင့် Pixel နေရာမပြောင်းပါ။

## ပြဿနာအဖြေရှာရန် {#troubleshooting}

Import လုပ်ထားသောပုံ ကြီးလွန်းလျှင် Handle များ Workspace အပြင်ရောက်နေနိုင်သည်။ **Layer → Fit Layer to Canvas** က Transparent margin များအပါအဝင် Layer အပြည့်ကို Scale လုပ်ပြီး အလယ်ချသည်။ Scale Layer သုံးနေစဉ်ဆို Preview ကို ပြင်ပေးသည်။ အခြား Tools မှဆို ချက်ချင်း Apply လုပ်သည်။ Canvas ကို ချဲ့လိုလျှင် **Image → Canvas Size…** သုံးပါ။

## ဆက်ဖတ်ရန် {#related}

- [ပုံများကို Layer အဖြစ် Import လုပ်ရန်](../../image/import-layers/)
- [Layer Transform](../../reference/transforms/)
- [Layer Move သို့မဟုတ် Transform လုပ်မရခြင်း](../../troubleshooting/moving/)
