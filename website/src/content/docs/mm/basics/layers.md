---
title: "Layers နှင့် Transparency"
description: "Active layer၊ Layer order၊ Transparency၊ Opacity၊ Lock နှင့် Movement Link အကြောင်း။"
translationKey: "basics/layers"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## ပြင်မည့် Layer ရွေးရန် {#active-layer}

Document ထဲတွင် Layers ကို အပေါ်အောက် စီထားသည်။ အပေါ် Layer ၏ Transparent နေရာမှ အောက် Layers ကို မြင်ရသည်။ ပေါင်းစပ်ပုံတွင် Transparent ဖြစ်နေသော နေရာကို Checkerboard ဖြင့် ပြသည်။

**Layers** ထဲရှိ Row ကို နှိပ်ပြီး Active လုပ်ပါ။ ပုံဆွဲခြင်း၊ Fill နှင့် Effect အများစုက ထို Layer ကိုသာ သက်ရောက်သည်။ မြင်ရသော Layers အားလုံးကို ပြင်ခြင်း မဟုတ်ပါ။ Document တစ်ခုတည်းထဲ အခြားပုံထည့်ရန် **File → Open as Layers…** ကို သုံးပါ။

## ပုံမြင်ရပုံ ပြင်ရန် {#appearance}

1. Row ရှိ Visibility ကို ပိတ်ဖွင့်ပြီး ထို Layer ပါ/မပါ နှိုင်းယှဉ်ပါ။
2. Layer တစ်ခုလုံး၏ Opacity လျှော့ရန် **Layer → Layer Opacity…** ကို သုံးပါ။
3. အပေါ်အောက် အစီအစဉ်ကို **Layer → Raise Layer** သို့မဟုတ် **Lower Layer** ဖြင့် ပြောင်းပါ။ အပေါ် Layer ၏ Opaque pixels က အောက် Layer ကို ဖုံးသည်။
4. ထပ်နေသော Colors ရောစပ်ပုံပြောင်းရန် Layers panel မှ Blend mode ကို ရွေးပါ။ **Normal** က သာမန် Layer ထပ်ပုံအတိုင်း ပြသည်။ **Multiply**၊ **Screen** နှင့် **Overlay** က Colors ကို မတူညီသော နည်းဖြင့် ရောစပ်သည်။

**ရလဒ်:** Canvas ပေါ်ရှိ ပေါင်းစပ်ပုံ ပြောင်းသော်လည်း Layer တစ်ခုစီ သီးခြား ကျန်မည်။ Layers ကို ထိန်းထားရန် `.lime` ဖြင့် Save လုပ်ပါ။ Export က Layers ပေါင်းထားသော ပုံတစ်ပုံ ထုတ်ပေးသည်။

## Lock နှင့် အတူရွှေ့ခြင်း {#locks-links}

- **Alpha Lock** က Raster layer ၏ ရှိပြီးသား Transparency ကို ထိန်းသည်။ ရှိပြီးသား Pixel များ၏ Color ကို ပြောင်းနိုင်သော်လည်း Eraser ဖြင့် မဖယ်နိုင်ပါ။ Editable Text တွင် သုံးမရပါ။
- Row တစ်ခုစီရှိ Padlock သည် **Position Lock** ဖြစ်သည်။ Layer ကို ကိုယ်တိုင်ရွှေ့ခြင်းနှင့် Transform လုပ်ခြင်းကို ပိတ်သည်။ Image တစ်ခုလုံး ပြောင်းသော Operations က Lock ရှိသော်လည်း သက်ရောက်သည်။
- Chain ခလုတ်ဖြစ်သော **Movement Link** ဖွင့်ထားသည့် Layers က အတူရွှေ့သည်။ တစ်ခုရွှေ့လျှင် အခြားဖွင့်ထားသော Layers၊ Hidden layers ပါ လိုက်သည်။ ပါဝင်သော Layer တစ်ခု Position Lock ရှိလျှင် အားလုံး ရွှေ့မရပါ။

Shadow နှင့် Outline layer အသစ်များက Source ကို လိုက်ရွှေ့ရန် ချိတ်ထားသည်။ Source ရွှေ့လျှင် ထို Layers ပါ လိုက်သည်။ Shadow ကို တိုက်ရိုက်ရွှေ့လျှင် Source နှင့် အကွာအဝေးကို ပြောင်းသည်။ သီးခြားဖြစ်စေရန် **Layer → Detach from Source** ကို သုံးပါ။ နောက်မှ Source ပြင်လည်း Shadow pixels ကို အလိုအလျောက် ပြန်မဖန်တီးပါ။

## ဆက်ဖတ်ရန် {#related}

- [ပုံများ ပေါင်းပြီး နေရာချရန်](../../guides/combine-images/)
- [Layer mask သုံးရန်](../../basics/masks/)
- [Layer ရွှေ့မရ၊ Transform မရလျှင်](../../troubleshooting/moving/)
