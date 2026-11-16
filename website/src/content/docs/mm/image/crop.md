---
title: "ပုံကို Crop လုပ်ရန်"
description: "Free သို့မဟုတ် Fixed ratio၊ Coordinates နှင့် Selection bounds ဖြင့် Canvas ကို Crop လုပ်ရန်။"
translationKey: "image/crop"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## ချန်ထားမည့် နေရာရွေးရန် {#steps}

Image တစ်ခုလုံး Crop လုပ်ရန် Hidden text layers အပါအဝင် Editable Text layers အားလုံးကို Rasterize လုပ်ထားရမည်။ Editable `.lime` မူရင်းကို အရင် Save လုပ်ပါ။ ပြီးလျှင် Text layer တစ်ခုစီကို ရွေးပြီး **Layer → Rasterize Text Layer** လုပ်ပါ။ မူရင်း Project ကို ဆက်ပြင်နိုင်အောင် ထားလိုလျှင် Flattened copy ကို Export လုပ်ပြီး ထိုမိတ္တူကို ပြန်ဖွင့်ကာ Crop လုပ်နိုင်သည်။ Editable Text ရှိစဉ် **Image → Canvas Size…** ကို သုံးနိုင်သေးသည်။ Canvas Size က ဘောင်ကိုသာ ပြောင်းပြီး Canvas အပြင်ဘက် Layer pixels ကို မဖျက်ပါ။

1. Toolbox မှ **Crop** ကို ရွေးပါ သို့မဟုတ် Canvas ပေါ်တွင် **C** နှိပ်ပါ။
2. **Tool Options** တွင် **Free**၊ **1:1**၊ **4:3**၊ **16:9**၊ **Image** သို့မဟုတ် **Ratio W/H** ဖြင့် Custom ratio ကို ရွေးပါ။ **Image** က လက်ရှိပုံ၏ Aspect ratio ကို သုံးသည်။
3. ချန်ထားမည့် နေရာကို ဆွဲပါ။ အတွင်းမှ ဆွဲပြီး ရွှေ့ပါ။ Square handle မှ ဆွဲပြီး အရွယ်ပြင်ပါ။ Fixed ratio က Crop အသစ်ဆွဲခြင်း၊ Handle ဆွဲခြင်းနှင့် Dimensions ရိုက်ခြင်းတို့ကို သတ်မှတ်ထားသော အချိုးအတိုင်း ထိန်းပေးသည်။
4. တိကျသော Bounds အတွက် **X**၊ **Y**၊ **Width** နှင့် **Height** ကို ရိုက်ပါ။ Coordinates က Image pixels ကို သုံးသည်။ တန်ဖိုးပြင်လျှင် Crop draft က ချက်ချင်း ပြောင်းသည်။ စာသားအလွတ်ထားလျှင် Crop မပြောင်းပါ။
5. **Image → Apply Crop** ကို ရွေးပါ သို့မဟုတ် Canvas ကို Focus ထားပြီး **Return** နှိပ်ပါ။ Draft ကို Cancel လုပ်ရန် Canvas ပေါ်တွင် **Escape** နှိပ်ပါ။

**ရလဒ်:** Document canvas က Crop area အရွယ် ဖြစ်လာမည်။ Canvas အသစ်အပြင်ဘက် Content ကို Crop က ဖယ်သည်။ ပြန်ယူရန် **Edit → Undo** ကို သုံးပါ။ ရလဒ်မှန်လျှင် Editable `.lime` project ကို Save လုပ်ပါ။

## Ratio ပြောင်းခြင်းနှင့် အပြင်မှ ဆွဲခြင်း {#geometry}

Fixed ratio ရွေးလျှင် လက်ရှိ Crop ကို ဘောင်ဟောင်းအတွင်း Center ထားပြီး ပြန်ညှိသည်။ Dimensions ကို Pixel အလိုက် Round လုပ်သည်။ **Free** က လက်ရှိ Shape မပြောင်းဘဲ Ratio ကန့်သတ်မှုကို ဖယ်သည်။ Custom ratio တွင် အပေါင်းကိန်းပြည့် W/H ကို သုံးပါ။

Crop ကို Canvas အပြင်မှ စဆွဲပြီး အတွင်းသို့ ဝင်နိုင်သည်။ Crop bounds က Image pixels အတွင်းသာ ရှိမည်။ ဆွဲထားသော Area က ပုံနှင့် လုံးဝမထပ်လျှင် ရှိပြီးသား Crop မပြောင်းပါ။

## အခြား Crop commands {#commands}

- **Image → Crop to Selection** က လက်ရှိ Selection ကို ဝန်းရံသော Rectangle ကို သုံးသည်။ Selection outline အတိုင်း Transparent ပုံဖြတ်ပေးခြင်း မဟုတ်ပါ။
- **Image → Crop to Content** က Canvas ကို Nontransparent content ရှိသည့် Bounds အထိ ကျုံ့သည်။
- **Layer → Crop to Content** က Active layer တစ်ခုကိုသာ Layer ၏ Opaque content အထိ Crop လုပ်သည်။ Canvas မပြောင်းပါ။ Position Lock ရှိလျှင် သုံးမရပါ။

Pixel မဖျက်ဘဲ နောက်ခံဖုံးလိုလျှင် Crop အစား [Layer mask](../../basics/masks/) ကို သုံးပါ။

## ဆက်ဖတ်ရန် {#related}

- [Image သို့မဟုတ် Canvas အရွယ်ပြင်ရန်](../../image/resize/)
- [Selection၊ Antialias နှင့် Feather](../../basics/selections/)
- [Preview၊ Apply နှင့် Cancel](../../basics/previews/)
