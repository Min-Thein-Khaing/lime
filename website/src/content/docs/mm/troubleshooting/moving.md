---
title: "Layer Move သို့မဟုတ် Transform လုပ်မရခြင်း"
description: "Lock၊ Movement Link၊ မျက်နှာပြင်အပြင်ရောက်နေသော Handle နှင့် မပြီးသေးသော Preview စစ်နည်း။"
translationKey: troubleshooting/moving
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Layer နှင့် Lock စစ်ရန် {#locks}

1. ပြင်လိုသော Layer ကို ရွေးပြီး မြင်ရအောင် လုပ်ပါ။ နေရာရွှေ့ရန် **Move** (`M`) သုံးပါ။ Selection Tool က Selection ကိုသာ ရွှေ့သည်။
2. Row ၏ Position padlock သို့မဟုတ် **Layer → Toggle Position Lock** ကို စစ်ပါ။ Position Lock က Pointer move၊ X/Y၊ Arrow nudge၊ Alignment၊ Scale/Rotate/Flip/Perspective၊ Fit Layer to Canvas နှင့် Layer content Crop ကို တားသည်။
3. Movement Link ရှိလျှင် Chain ဖွင့်ထားသော Layer အားလုံးနှင့် နောက်လိုက်တွဲထားသော Attached Layer များကို စစ်ပါ။ ပါဝင်သော Layer တစ်ခု Lock ဖြစ်လျှင် အစုလိုက်ရွှေ့မရပါ။ Hidden Layer ဖြစ်လည်း ပါသည်။ Pointer သို့မဟုတ် Arrow ဖြင့် ရွှေ့ခြင်းကို Lock က တားလျှင် Lime က ထို Layer အမည်ကို ပြသည်။
4. Editable Text ကို Scale Layer နှင့် အခြား Pixel Transform လုပ်ရန် `.lime` Copy ထားပြီး **Layer → Rasterize Text Layer** လုပ်ပါ။ Text ကို Move လုပ်ရုံအတွက် Rasterize မလိုပါ။

## လက်ရှိ Edit အဆုံးသတ်ရန် {#preview}

Draft ကြောင့် မရနိုင်သော Command သုံးမီ ပြောင်းထားသော Crop/Gradient/Transform Preview ကို Apply သို့မဟုတ် Cancel လုပ်ပါ။ Row metadata ပြောင်းခြင်း သို့မဟုတ် Right sidebar ခေါက်ခြင်းမတိုင်မီ Pointer drag အဆုံးသတ်ပါ။

Numeric field အတွင်း Return က ထို Field ကိုသာ အဆုံးသတ်ပြီး Transform တစ်ခုလုံးကို Apply မလုပ်ပါ။ **Apply** Click လုပ်ပါ၊ သို့မဟုတ် Canvas Focus ဖြင့် Return နှိပ်ပါ။ Field ကို အလွတ်ထားခြင်း၊ မမှန်သောတန်ဖိုးနှင့် Escape က Typed edit ပြန်ထားသည်။ လက်ရှိ Transform ပြန်ရွေးလျှင် Preview ကျန်သည်။ Tool/Layer/Tab ပြောင်းလျှင် Cancel လုပ်သည်။

## Layer သို့မဟုတ် Handle ရှာရန် {#geometry}

Active Layer ၏ စတုဂံမြင်ရန် **View → Toggle Layer Boundary** သုံးပါ။ Layer ကြီးလျှင် Canvas အပြင်ရောက်နေနိုင်သည်။ Scale Layer တွင် **Fit Layer to Canvas** က Transparent margin များအပါအဝင် Raster အပြည့်ကို အချိုးထိန်းပြီး အလယ်ချသော Preview ပြသည်။ အခြား Tools မှဆို ချက်ချင်း Apply လုပ်သည်။

တိကျစွာရွှေ့ရန် Move တွင် အပေါင်း/အနုတ် X/Y ရိုက်ပြီး Field အဆုံးသတ်ပါ။ Arrow key က ပုံ၏ Pixel တစ်ခု၊ Shift နှင့်ဆို ဆယ်ခု ရွှေ့သည်။ Numeric placement နှင့် Nudge က Snapping ကို မသုံးပါ။ မရည်ရွယ်ဘဲ Snap ဖြစ်လျှင် Snap to Grid/Guides ပိတ်ပါ၊ သို့မဟုတ် Pointer drag လုပ်စဉ် Control ဖိပါ။

## Movement Link နှင့် Attachment ခွဲရန် {#attachments}

Chain toggle များက Document အတွက် Movement Link အစုတစ်ခု သတ်မှတ်သည်။ ထိုအစုမှ Layer တစ်ခုထုတ်ရန် ထို Layer ၏ Toggle ပိတ်ပါ။ Attached Drop Shadow/Outline များက Chain ပိတ်ထားလည်း Source နောက်လိုက်သည်။ ထိုချိတ်ဆက်မှုကို **Layer → Detach from Source** ဖြင့် ဖယ်ပါ။ Attached Effect ကို တိုက်ရိုက်ရွှေ့လျှင် Relative offset ပြောင်းပြီး Source Layer ကို ပြန်မရွှေ့ပါ။

Image တစ်ခုလုံး Transform လုပ်ခြင်းက Locked Layer များကိုလည်း သက်ရောက်သည်။ Document အားလုံးပြောင်းလိုမှ Image Menu သုံးပါ။

## ဆက်ဖတ်ရန် {#related}

- [ပုံများပေါင်းပြီး နေရာချရန်](../../guides/combine-images/)
- [Layer Transform](../../reference/transforms/)
- [Preview နှင့် Undo](../../basics/previews/)
