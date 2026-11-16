---
title: "ပုံအလွတ်မှ စတင်ရန်"
description: "Canvas ဖန်တီးပြီး နောက်ခံ၊ ပုံနှင့် စာတန်းကို သီးခြား Layer များဖြင့် ထည့်ရန်။"
translationKey: start/new-image
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
---

## Canvas ရွေးရန် {#canvas}

ပုံဆွဲခြင်း၊ စာတန်းကတ် သို့မဟုတ် ပုံများပေါင်းထားသော Design အတွက် ပုံအလွတ်မှ စနိုင်သည်။ ဥပမာ နောက်ခံအရောင်နှင့် ခေါင်းစဉ်ပါသော 1,200 × 800 Pixel ကတ်တစ်ခု ဖန်တီးကြည့်ပါ။

1. **File → New Image…** ကို ရွေးပါ။
2. **Width (px)** တွင် 1200၊ **Height (px)** တွင် 800 ရိုက်ပါ၊ သို့မဟုတ် လိုအပ်သော အရွယ်ကို သုံးပါ။ ဤတန်ဖိုးများက Export ပုံ၏ Pixel အရွယ်ဖြစ်သည်။ Screen ပေါ်တွင် မြင်ရသော အရွယ် မဟုတ်ပါ။
3. **Canvas background** တွင် **Transparent** ရွေးပြီး **Create** နှိပ်ပါ။ Checkerboard က နောက်ခံပုံ မရှိသေးကြောင်း ပြသည်။ Export ပုံထဲ မပါပါ။

လက်ရှိ Background swatch ဖြင့် ပုံအသစ်ကို ဖြည့်လိုလျှင် **Background color** ရွေးနိုင်သည်။ ပုံမဖန်တီးမီ Swatch ပြင်ရန် [Color ရွေးပြီး ပြန်သုံးရန်](../../basics/colors/) ကို ကြည့်ပါ။

## နောက်ခံကို သီးခြားထည့်ရန် {#background}

1. Transparent ပုံအသစ်တွင် Layer တစ်ခု ပါသည်။ ပုံဆွဲရန် သီးသန့်ထားဖို့ **Layer → Rename Layer…** ဖြင့် **Drawing** ဟု အမည်ပေးပါ။
2. ကတ်နောက်ခံအတွက် Foreground color ရွေးပါ။
3. **Layer → New Layer…** ကို ရွေးပြီး **Background** ဟု အမည်ပေးပါ။ Fill တွင် **Foreground** ရွေးပြီး **Create** နှိပ်ပါ။
4. Background ကို Drawing အောက်သို့ ချရန် **Layer → Lower Layer** သုံးပါ။ Row များကို ဆွဲပြီးလည်း အစီအစဉ်ပြောင်းနိုင်သည်။

Fill က Layer ဖန်တီးချိန်ရှိ Swatch color ကို သုံးသည်။ နောက်မှ Swatch ပြောင်းရုံဖြင့် ရှိပြီးသား နောက်ခံအရောင် မပြောင်းပါ။ အောက် Layers ကို မဖုံးဘဲ ပုံဆွဲလိုလျှင် **Transparent** Layer အသစ် သုံးပါ။

## ပုံနှင့် စာတန်းထည့်ရန် {#artwork}

1. ပုံဆွဲရန် **Drawing** ရွေးပြီး နောက်ခံနှင့် ကွဲပြားထင်ရှားသော Foreground color ရွေးပါ။ Brush သို့မဟုတ် Pencil ဖြင့် ပုံဆွဲပါ။
2. ပုံဖိုင်ထည့်လိုလျှင် **File → Open as Layers…** သုံးပါ။ ကတ်နှင့် လိုက်ဖက်အောင် [Scale လုပ်ပြီး နေရာချပါ](../../guides/combine-images/)။
3. **Text** (`T`) ရွေးပြီး နေရာလွတ်ကို နှိပ်ကာ ခေါင်းစဉ်ရိုက်ပါ။ Tool Options တွင် Font၊ Size နှင့် Color ရွေးပြီး **Cmd/Ctrl+Enter** ဖြင့် အဆုံးသတ်ပါ။ Text အသစ်က Layers အားလုံး၏ အပေါ်တွင် ပေါ်သည်။
4. **Move** ဖြင့် Text ကို နေရာချပါ။ မဆွဲမီ Layers တွင် ပြင်မည့် Row ကို ရွေးပါ။

**ရလဒ်:** နောက်ခံ၊ ပုံနှင့် Text က သီးခြား Layers ဖြစ်မည်။ နောက်ခံကို ပြန်မဆွဲဘဲ ခေါင်းစဉ်ပြင်ခြင်း၊ ပုံရွှေ့ခြင်း လုပ်နိုင်သည်။

## Save လုပ်ပြီး မျှဝေရန် {#save}

**File → Save Project As…** ဖြင့် **`.lime`** မူရင်း သိမ်းပါ။ မျှဝေရန် **File → Export…** မှ PNG မိတ္တူ ထုတ်ပါ။ PNG က Transparent နေရာများကို ထိန်းထားသည်။ ကတ်တွင် နောက်ခံအပြည့် ရှိစေလိုလျှင် Opaque နောက်ခံ Layer ကို ဖွင့်ထားပါ။

## ဆက်ဖတ်ရန် {#related}

- [Color ရွေးပြီး ပြန်သုံးရန်](../../basics/colors/)
- [Layers နှင့် Transparency](../../basics/layers/)
- [Text၊ Drop Shadow နှင့် Outline ထည့်ရန်](../../guides/text-effects/)
