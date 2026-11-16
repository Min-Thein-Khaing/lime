---
title: "Color နှင့် Gradient ဖြည့်ရန်"
description: "Connected region သို့မဟုတ် Selection အတွင်း Color ဖြည့်ခြင်းနှင့် Gradient Preview လုပ်နည်း။"
translationKey: guides/fills-gradients
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
---

## Color ဖြည့်ရန် {#steps}

1. ပြင်လိုသော မြင်ရသည့် Raster Layer ကို ရွေးပြီး [Foreground color သတ်မှတ်ပါ](../../basics/colors/)။
2. **Bucket Fill** (`B`) ရွေးပါ။ ဆက်နေသော Color တူသည့်နေရာအတွက် **Connected** ရွေးပါ။ လက်ရှိ Selection အပြည့် ဖြည့်ရန် **Selection** ရွေးပါ။
3. Connected တွင် Color အတိအကျတူမှ ဖြည့်ရန် **Tolerance** 0 ဖြင့် စပါ။ ဆင်တူသော Color များပါ ဖြည့်လိုလျှင် တိုးပါ။ **Gap radius** နည်းနည်းက နယ်နိမိတ်ရှိ အပေါက်ငယ်များကို ပိတ်တွက်ပေးသည်။ နယ်နိမိတ် Pixel များကို မဖြည့်ပါ။
4. အခြား Layer များနှင့် ပေါင်းမှ နယ်နိမိတ်ပြည့်စုံလျှင် **Sample All Layers** ဖွင့်ပါ။ Fill က Active Layer ပေါ်သာ သက်ရောက်သည်။
5. Canvas အတွင်း Click လုပ်ပါ။ ရည်ရွယ်သည်ထက် ပိုကျယ်သွားလျှင် Undo လုပ်ပြီး Settings ပြင်ကာ ပြန်စမ်းပါ။

## ရလဒ် {#result}

Connected region သို့မဟုတ် Selection အတွင်း Foreground Color ဖြည့်မည်။ Active Layer နှင့် Selection ကန့်သတ်ချက်အတွင်းသာ သက်ရောက်သည်။ Selection mode တွင် Matching နှင့် Sampling controls များကို မသုံးပါ။ Bucket Fill နှင့် Magic Wand တို့၏ Tolerance၊ Gap Settings ကို သီးခြားမှတ်ထားသည်။

## Gradient Preview လုပ်ရန် {#gradient}

1. **Gradient** (`G`) ရွေးပြီး Canvas ပေါ်တွင် မျဉ်းတစ်ကြောင်း ဆွဲပါ။
2. **FG → BG**၊ **FG → Clear** သို့မဟုတ် **Custom** ရွေးပါ။ Foreground/Background mode များက လက်ရှိ Swatch များကို သုံးသည်။ Custom က သီးခြား RGBA stop များကို သုံးသည်။
3. အစနှင့် အဆုံးမှတ်ကို နေရာချပြီး Linear/Radial ပုံစံ ရွေးပါ။ **Reverse** က ထိုမှတ်များ မရွှေ့ဘဲ Color အစီအစဉ်ကို ပြောင်းပြန်လုပ်သည်။
4. Custom တွင် Stop ရွေးပြီး RGB၊ Opacity နှင့် Position ပြင်ပါ။ **Add Stop** က Stop များကြား အကျယ်ဆုံးနေရာကို ပိုင်းပေးသည်။ Stop ရှစ်ခုအထိ သုံးနိုင်ပြီး အစနှင့် အဆုံးကို 0%၊ 100% တွင် ထားသည်။
5. **Apply** လုပ်ပါ၊ သို့မဟုတ် Canvas ပေါ်တွင် Return နှိပ်ပါ။ **Cancel** သို့မဟုတ် Escape က Preview ကို ပယ်ဖျက်သည်။

## Gradient Preset သိမ်းရန် {#presets}

လက်ရှိ Color၊ ပုံစံနှင့် Direction ကို **Save…** ဖြင့် သိမ်းပါ။ **Saved gradients** ကို ဖြန့်ပြီး Preset များ Load၊ Rename၊ Delete လုပ်နိုင်သည်။ Load လုပ်ခြင်းက အစအဆုံးမှတ်နှင့် Foreground/Background swatch များကို မပြောင်းပါ။ Personal preset 32 ခုအထိ သိမ်းနိုင်ပြီး App ပြန်ဖွင့်လည်း ကျန်သည်။ Project Undo History နှင့် သီးခြားဖြစ်သည်။

Library သိမ်းရာတွင် Error ဖြစ်လျှင် လက်ရှိ Draft ကို ဆက်ထားပြီး Library ထပ်မပြင်မီ **Reload Library** သုံးပါ။ သိမ်းမရလျှင် ယခင် Library ကို အစားမထိုးပါ။

## ဆက်ဖတ်ရန် {#related}

- [Selection၊ Antialias နှင့် Feather](../../basics/selections/)
- [ပုံဆွဲခြင်း သို့မဟုတ် Fill လုပ်သော်လည်း မပြောင်းခြင်း](../../troubleshooting/painting/)
- [Preview နှင့် Undo](../../basics/previews/)
