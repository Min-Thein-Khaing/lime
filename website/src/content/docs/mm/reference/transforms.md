---
title: "Layer Transform"
description: "Layer တစ်ခုကို Scale၊ Rotate၊ Flip နှင့် Perspective Preview လုပ်နည်း။"
translationKey: reference/transforms
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

**Layer → Transform**၊ **Tools → Transform** သို့မဟုတ် Toolbox မှ Transform ရွေးပါ။ Active visible Raster Layer ကို Preview ပြသည်။ Editable Text ကို အရင် Rasterize လုပ်ရသည်။ Position Lock ရှိလျှင် ဤလုပ်ဆောင်ချက်များ မရပါ။

## Preview အသုံးပြုပုံ {#preview}

**Apply** သို့မဟုတ် Canvas ပေါ် Return က Preview အားလုံးကို Undo တစ်ဆင့်အဖြစ် Commit လုပ်သည်။ **Cancel** သို့မဟုတ် Canvas ပေါ် Escape က ပယ်ဖျက်သည်။ Numeric field အတွင်း Return က ထို Edit ကိုသာ အဆုံးသတ်သည်။ Transform တစ်ခုလုံးကို Apply မလုပ်ပါ။ Field ကို အလွတ်ထားခြင်း သို့မဟုတ် Escape နှိပ်ခြင်းက Numeric edit ပြန်ထားသည်။

အကြိမ်ကြိမ်ဆွဲလည်း နောက်ဆုံး Apply လုပ်ချိန် မူရင်း Source ကို သုံးသည်။ လက်ရှိ Tool ကို ပြန်ရွေးလျှင် Preview ကျန်သည်။ Tool၊ Tab သို့မဟုတ် Layer ပြောင်းလျှင် Cancel လုပ်သည်။ Apply/Cancel ပြီးနောက် Tool ပြန်ရွေးလျှင် Session အသစ်စသည်။ Scale Layer တွင် Canvas Click ဖြင့်လည်း ပြန်စနိုင်သည်။

Layer Transform လုပ်စဉ် Selection က Canvas ပေါ်ရှိ မူလနေရာတွင် ကျန်သည်။

## Scale Layer {#scale}

Key **Shift+S** ဖြစ်သည်။ ဘေး သို့မဟုတ် ထောင့် Handle ဆွဲပါ၊ သို့မဟုတ် Width/Height ရိုက်ပါ။ Aspect lock ကို မူလဖွင့်ထားသည်။ ဆွဲနေစဉ် Shift က ထို Settings ကို ယာယီပြောင်းပြန်လုပ်သည်။ ကိုးမှတ်ပါ **Size anchor** က Width/Height ရိုက်ပြီး Resize လုပ်မည့် အခြေကို ရွေးပေးသည်။ Handle ဆွဲလျှင် ဆန့်ကျင်ဘက်အစွန်း သို့မဟုတ် ထောင့်ကို နေရာမပြောင်းဘဲ ထားသည်။

**Nearest** က မူရင်း Pixel ၏ Hard edge ကို ထိန်းသည်။ **Bilinear** နှင့် **Bicubic** က Pixel ကြား Color ကို တွက်ပေးသည်။ ရွေးထားသော Method ကို Apply တွင် သုံးသည်။ Preview resolution က ကန့်သတ်ထားသဖြင့် အသေးစိတ်စစ်ရန် Apply ပြီးရလဒ်ကို 100% ဖြင့် ကြည့်ပါ။

**Fit Layer to Canvas** က Transparent margin များအပါအဝင် Raster အပြည့်ကို အချိုးထိန်းပြီး Scale လုပ်ကာ အလယ်ချသည်။ ပုံငယ်ကိုလည်း ချဲ့နိုင်သည်။ Scale Layer သုံးနေစဉ်ဆို Draft ပြင်ပေးသည်။ အခြားနေရာမှဆို ချက်ချင်း Apply လုပ်သည်။ **Layer to Canvas Size** က Pixel ကို Scale မလုပ်ဘဲ Crop/Pad လုပ်သည်။

## Rotate နှင့် Flip {#rotate-flip}

**Rotate** သည် Shift+R ဖြစ်သည်။ ဆွဲပါ၊ သို့မဟုတ် Angle ကို Degree ဖြင့် ရိုက်ပါ။ Pivot anchor ကိုးခုမှ ရွေးပါ၊ Canvas X/Y ကို အပေါင်း/အနုတ်ဖြင့် ရိုက်ပါ၊ သို့မဟုတ် စက်ဝိုင်း Crosshair ဆွဲပါ။ Shift-drag က ထို Pivot ပတ်လည် 15-degree အဆင့်များတွင် Snap လုပ်သည်။

**Flip** သည် Shift+F ဖြစ်သည်။ Horizontal/Vertical ရွေးပါ၊ သို့မဟုတ် ဆွဲပြီး Direction ရွေးကာ Apply လုပ်ပါ။

## Perspective {#perspective}

Key **Shift+P** ဖြစ်သည်။ ထောင့် Grip လေးခု ဆွဲပါ၊ သို့မဟုတ် Corner selector မှ တစ်ခုရွေးပြီး Canvas X/Y ရိုက်ပါ။ **Reset Corner** က ထိုမှတ်ကိုသာ ပြန်ထားသည်။ **Reset All** က မူရင်းစတုဂံ ပြန်ထားသည်။ ထောင့်များဖြတ်ကျော်နေခြင်း သို့မဟုတ် ရလဒ်အလွန်ကြီးခြင်းကြောင့် Apply ပိတ်နေလျှင် Draft ကို ဆက်ပြင်နိုင်သည်။

ပုံတစ်ခုလုံး Quarter-turn နှင့် Flip အတွက် **Image → Transform** သုံးပါ။ Layer အားလုံးကို သက်ရောက်သည်။ Document အပြည့် Scale လုပ်ရန် **Image → Scale Image…** သုံးပါ။

## ဆက်ဖတ်ရန် {#related}

- [ပုံ သို့မဟုတ် Canvas ကို Resize လုပ်ရန်](../../image/resize/)
- [ပုံများပေါင်းပြီး နေရာချရန်](../../guides/combine-images/)
- [Layer Move သို့မဟုတ် Transform လုပ်မရခြင်း](../../troubleshooting/moving/)
