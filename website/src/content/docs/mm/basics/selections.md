---
title: "Selection၊ Antialias နှင့် Feather"
description: "ပုံ၏ တစ်စိတ်တစ်ပိုင်းကိုသာ ပြင်ရန် Selection သုံးပြီး Antialias၊ Feather နှင့် ပေါင်းစပ်ရွေးချယ်နည်း သိရန်။"
translationKey: "basics/selections"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Selection အလုပ်လုပ်ပုံ {#meaning}

Selection က ပုံဆွဲခြင်း၊ Fill နှင့် Effect အများစုတွင် ပြင်နိုင်သော Pixel နေရာကို ကန့်သတ်သည်။ Active layer ကို ရွေးပေးခြင်း မဟုတ်ပါ။ Soft selection တွင် Pixel တစ်ခုကို တစ်စိတ်တစ်ပိုင်းသာ သက်ရောက်နိုင်သည်။ ထို့ကြောင့် Selection အနားဘက်တွင် Edit ၏ သက်ရောက်မှု တဖြည်းဖြည်း လျော့သွားသည်။

Selection outline က ဘောင်ကိုသာ ပြသည်။ အနားရှိ Partial coverage အားလုံးကို မပြနိုင်သဖြင့် ရလဒ်ကို Zoom ချဲ့ပြီး စစ်ပါ။ Selection ပြောင်းခြင်းကို Undo လုပ်နိုင်သည်။ Selection သာ ပြောင်းလျှင် သိမ်းထားပြီးသား Project ကို Unsaved ဟု မမှတ်ပါ။

## Selection ဖန်တီးပြီး ပေါင်းစပ်ရန် {#steps}

1. **Select** menu သို့မဟုတ် Toolbox မှ **Rectangle Select**၊ **Oval Select**၊ **Free Select** သို့မဟုတ် **Magic Wand** ကို ရွေးပါ။
2. Rectangle/Oval ကို ဆွဲပါ၊ Free outline ကို ပိတ်ပါ သို့မဟုတ် Wand ဖြင့် ဆက်နေသော နေရာကို နှိပ်ပါ။
3. နောက်ထပ် Selection လုပ်စဉ် **Shift** ကို ဖိထားပြီး ထပ်ပေါင်းပါ။ **Control** ဖြင့် နုတ်ပါ။ **Shift+Control** ဖြင့် နှစ်ခုထပ်နေသော နေရာကိုသာ ယူပါ။ macOS တွင်လည်း ဤ Gesture များအတွက် Control ကို သုံးသည်။
4. Active layer ပေါ်တွင် ပုံဆွဲပါ သို့မဟုတ် Fill လုပ်ပါ။ အခြားနေရာပြင်ရန် ကန့်သတ်မှုဖယ်လိုလျှင် **Select → Select None** ကို ရွေးပါ။

စက်ဝိုင်းအတွက် Oval Select တွင် **1:1** ရွေးပါ။ Rectangle နှင့် Oval က Aspect ratio တစ်ခုတည်းကို မျှဝေသုံးသည်။ Crop က သီးခြား မှတ်ထားသည်။ Fixed ratio ရွေးလျှင် လက်ရှိ Editable shape ကိုလည်း Center မပြောင်းဘဲ ပြန်ညှိပေးသည်။

## Antialias နှင့် Feather {#soft-edges}

**Antialias edges** က Curve နှင့် စောင်းနေသော အနားကို ပါးလွှာသော Pixel fringe အတွင်း ချောမွေ့စေသည်။ **Feather new selections** က ပိုကျယ်သော Radius အတွင်း Coverage ကို ဖြန့်ပြီး Soft edges ဖန်တီးသည်။ နှစ်ခုက မတူသော Settings ဖြစ်ပြီး အတူသုံးနိုင်သည်။

ရှိပြီးသား Selection ကို **Select → Feather…** ဖြင့် Feather လုပ်ပါ။ **Grow…** က ချဲ့ပြီး **Shrink…** က ကျုံ့သည်။ Soft coverage ကို ထိန်းထားသည်။ Selection အသစ်အတွက် Feather setting ပြောင်းရုံဖြင့် လက်ရှိ Selection မပြောင်းပါ။

Shapes ပေါင်းစပ်ပြီးနောက် သို့မဟုတ် Coverage ပြင်ပြီးနောက် Selection က Mask ဖြစ်သွားနိုင်သည်။ ထိုအခါ မူလ Rectangle/Oval controls မရှိတော့နိုင်ပါ။ Selection mask က ပြင်နိုင်သော နေရာကို ကန့်သတ်သည်။ Layer mask က Layer ၏ Visibility ကို ထိန်းသည်။

## ဆက်ဖတ်ရန် {#related}

- [Selection tools](../../reference/selection/)
- [နောက်ခံဖယ်ရန်](../../guides/remove-background/)
- [Layer mask သုံးရန်](../../basics/masks/)
