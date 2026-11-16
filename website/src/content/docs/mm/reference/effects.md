---
title: "Color Adjustment နှင့် Effects"
description: "Color ပြောင်းမှု Preview၊ သိမ်းထားသော Layer Effect ပြန်ပြင်ခြင်းနှင့် Settings ထပ်သုံးနည်း။"
translationKey: reference/effects
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
---

## Effect ရွေးရန် {#choose}

Adjustment ရွေးပြီး စပြင်ရန် [ပုံ၏ အလင်းအမှောင်နှင့် Color ပြင်ရန်](../../guides/color-corrections/) ကို ကြည့်ပါ။

**Colors** တွင် Brightness & Contrast၊ Hue & Saturation၊ Color Balance၊ Levels၊ Curves၊ Desaturate နှင့် Invert ပါသည်။ ဤ Adjustment များက Layer ၏ Alpha ကို မပြောင်းပါ။ Levels နှင့် Curves တွင် Channel controls ရှိသည်။ Histogram က လုံးဝ Transparent ဖြစ်သော Pixel များကို မတွက်ပါ။

**Filters** တွင် Blur၊ Sharpen၊ Drop Shadow နှင့် Stroke ပါသည်။ Blur/Sharpen က Raster Layer ပေါ်တွင် Selection အလိုက် အလုပ်လုပ်သည်။ Alpha Lock မရှိလျှင် Color နှင့် Alpha နှစ်မျိုးလုံးကို ရောပြောင်းသည်။ Drop Shadow/Stroke က မြင်ရသော Raster သို့မဟုတ် Editable Text Source အောက်တွင် Layer သီးခြားဖန်တီးသည်။

## Preview နှင့် Apply {#steps}

1. မြင်ရသော Layer ရွေးပါ။ တစ်စိတ်တစ်ပိုင်းသာ ပြင်လိုလျှင် Selection အရင်လုပ်ပါ။
2. Colors သို့မဟုတ် Filters မှ Command ဖွင့်ပါ။ Settings ပြင်စဉ် အခြား Layer၊ Mask နှင့် Opacity ပါသော ပေါင်းစပ်ပုံကို ကြည့်နိုင်သည်။
3. **Before** နှင့် **After** နှိုင်းယှဉ်ပါ။ Before က ကြည့်ပုံကိုသာ ပြောင်းသည်။ Apply လုပ်လျှင် ရွေးထားသော Parameter များကို အသုံးချသည်။
4. **Reset** က မူလ Default ပြန်ထားပြီး After သို့ ပြန်ပြောင်းသည်။ **Apply** က Effect ကို အတည်ပြုသည်။ **Cancel** က Pixel မပြောင်းဘဲ ပိတ်သည်။

ရလဒ်မပြောင်းသော Adjustment သို့မဟုတ် Strength သုည Effect က Image History မထည့်ပါ။ အောင်မြင်သော Parameter များကို Tab များနှင့် App restart ကြား မှတ်ထားသည်။ Cancel လုပ်ထားသော သို့မဟုတ် မအောင်မြင်သော Preview က ယခင်တန်ဖိုးများကို အစားမထိုးပါ။

## Layer Effect ပြန်ပြင်ရန် {#retained}

Raster Layer တစ်ခုက Color Adjustment သို့မဟုတ် Blur/Sharpen Source တစ်ခုကို နောက်မှပြန်ပြင်ရန် သိမ်းထားနိုင်သည်။ **Layer → Edit Layer Effect…** ဖြင့် ပြန်ဖွင့်ပါ။ **Remove Layer Effect** က သိမ်းထားသော Source ကို ပြန်ယူသည်။ **Bake Layer Effect** က လက်ရှိမြင်ရပုံကို ထားပြီး ပြန်ပြင်နိုင်သော Effect data ကို ဖယ်သည်။

ပုံဆွဲခြင်း သို့မဟုတ် Geometry ပြောင်းခြင်းက သိမ်းထားသော Effect ရလဒ်ကို Bake လုပ်သည်။ Effect အသစ်က လက်ရှိမြင်ရပုံကို Source အဖြစ် သုံးပြီး ယခင် Retained slot ကို အစားထိုးသည်။ Layer ရွှေ့ခြင်း၊ Opacity ပြောင်းခြင်းနှင့် Mask တိုက်ရိုက်ပြင်ခြင်းက Effect ကို ဆက်ထိန်းထားသည်။ ပြန်ပြင်နိုင်သော Data သိမ်းရန် `.lime` ဖြင့် Save လုပ်ပါ။

## Effect ထပ်သုံးရန် {#repeat}

**Filters → Repeat Last Effect** က နောက်ဆုံးအောင်မြင်သော Settings ကို လက်ရှိသုံးနိုင်သော Layer နှင့် Selection တွင် အသုံးချသည်။ **Reopen Last Effect…** က ထို Settings ဖြင့် Preview ပြန်ဖွင့်သည်။ Shortcuts များက Cmd/Ctrl+Alt+F နှင့် Cmd/Ctrl+Alt+Shift+F ဖြစ်သည်။

Drop Shadow နှင့် Stroke ကို Editable Text တွင် Rasterize မလုပ်ဘဲ သုံးနိုင်သည်။ ဖန်တီးထားသော Pixel များက Source ရွှေ့လျှင် လိုက်ရွှေ့သော်လည်း Source ပြင်လျှင် ပုံကို ပြန်မလုပ်ပါ။ Enabled Mask က Source silhouette ကို သက်ရောက်သည်။ Stroke က Outside Outline ဖွဲ့ရာတွင် Source Opacity နှင့် Selection ကို မသုံးပါ။ လုပ်ဆောင်ပုံအတွက် [Text၊ Drop Shadow နှင့် Outline လမ်းညွှန်](../../guides/text-effects/) ကို ကြည့်ပါ။

## ဆက်ဖတ်ရန် {#related}

- [Preview နှင့် Undo](../../basics/previews/)
- [Text၊ Drop Shadow နှင့် Outline ထည့်ရန်](../../guides/text-effects/)
- [Layers နှင့် Transparency](../../basics/layers/)
