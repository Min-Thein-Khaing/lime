---
title: "ပုံများပေါင်းပြီး နေရာချရန်"
description: "ပုံများကို Layer အဖြစ် Import လုပ်၍ Scale၊ Move နှင့် Movement Link သုံးနည်း။"
translationKey: guides/combine-images
sourceRevision: "1.2.3-guide-1"
translatedRevision: "1.2.3-guide-1"
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

## အခြားပုံ၏ တစ်စိတ်တစ်ပိုင်းကို Copy လုပ်ရန် {#clipboard}

1. မူရင်းပုံကို ဖွင့်ပြီး လိုချင်သော Pixel များပါသည့် Layer ရွေးပါ။ **Copy** က ထို Layer နှင့် ၎င်း၏ Enabled Mask ကိုသာ ယူသည်။ မြင်ရသော Layers အားလုံးကို ပေါင်းယူခြင်း မဟုတ်ပါ။
2. Copy လုပ်မည့်နေရာကို Rectangle၊ Oval သို့မဟုတ် Free Selection ဖြင့် ရွေးပါ။ Selection outline ကို ရွှေ့ခြင်းက ပြင်နိုင်သော ဘောင်ကိုသာ ရွှေ့သည်။ ပုံ၏ Pixel များ မလိုက်ပါ။
3. **Edit → Copy** (**Cmd/Ctrl+C**) သုံးပါ။ မူရင်းမှ ထို Pixel များကို ဖယ်လိုမှသာ **Edit → Cut** (**Cmd/Ctrl+X**) သုံးပါ။ Cut အတွက် မြင်ရသော Raster Layer ဖြစ်ပြီး Alpha Lock ပိတ်ထားရမည်။
4. Copy ပြီးလျှင် လက်ခံမည့် Document Tab ကို နှိပ်ပြီး **Edit → Paste** (**Cmd/Ctrl+V**) သုံးပါ။ Text ပြင်နေမှုကို အရင်အဆုံးသတ်ပါ။ Text draft ထဲရှိ Paste က စာထည့်သည်။
5. Copy ပုံက Canvas အရွယ်မပြောင်းဘဲ အလယ်တွင် Layer အသစ် ဖြစ်လာမည်။ **Move** ဖြင့် နေရာချပါ၊ သို့မဟုတ် **Scale Layer** ဖြင့် အရွယ်ပြင်ပြီး Apply လုပ်ပါ။

**ရလဒ်:** Copy လုပ်ထားသော အပိုင်းက မူရင်းနှင့် သီးခြားဖြစ်မည်။ Oval/Free Selection အပြင်ဘက်က Pasted Layer တွင် Transparent ဖြစ်ပြီး Feather လုပ်ထားသော အနားက Soft edges အတိုင်း ကျန်သည်။ Cut ကြောင့် မူရင်းပြောင်းလဲမှုကိုပါ သိမ်းလိုလျှင် Project နှစ်ခုလုံး Save လုပ်ပါ။

Copy လုပ်ထားသော အပိုင်းကို Document သီးခြား ဖန်တီးလိုလျှင် **File → New from Clipboard** သုံးပါ။ Document တစ်ခုတည်းအတွင်း Pixel ရွှေ့လိုလျှင် ထိုနေရာတွင် Copy၊ Paste လုပ်ပြီး Pasted Layer ကို ရွှေ့ပါ။

## Guides နှင့် Grid ဖြင့် ညှိရန် {#guides-grid}

ဥပမာ ပုံနှင့် စာတန်း၏ ဘယ်ဘက်အနားများ တန်းစေရန် Vertical Guide တစ်ခု ချထားနိုင်သည်။

1. **View → Toggle Rulers and Guides** ရွေးပြီး **Move** (`M`) သုံးပါ။
2. Vertical Guide အတွက် ဘယ် Ruler မှ Canvas ထဲသို့ ဆွဲပါ။ Horizontal Guide အတွက် အပေါ် Ruler မှ ဆွဲပါ။ Guide ကို ဆွဲပြီး နေရာပြောင်းနိုင်သည်။ ဖယ်ရန် Canvas Workspace အပြင်သို့ ဆွဲကာ Ruler ပေါ် သို့မဟုတ် Workspace အပြင်တွင် လွှတ်ပါ။
3. **View → Snap to Guides** ဖွင့်ပါ။ ပုံ၏ Layer ရွေးပြီး Guide အနီးသို့ ဆွဲပါ။ အနား သို့မဟုတ် အလယ်က Guide နှင့် တည့်အောင် Snap ဖြစ်နိုင်သည်။ စာတန်း Layer ကိုလည်း ထိုနည်းအတိုင်း ညှိပါ။
4. အကွာအဝေး ပုံမှန်ထားရန် **View → Toggle Grid** နှင့် **Snap to Grid** သုံးပါ။ Move ၏ **Snapping** section ထဲရှိ **Grid spacing (px)** တွင် လိုအပ်သော အကွာအဝေး သတ်မှတ်ပါ။

Drag လုပ်စဉ် **Control** ဖိထားလျှင် Snapping နှင့် Guide ဆွဲမိခြင်းကို ခဏကျော်နိုင်သည်။ macOS တွင်လည်း Control သုံးသည်။ X/Y ရိုက်ခြင်းနှင့် Arrow key ဖြင့် ရွှေ့ခြင်းကလည်း Snapping ကို မသုံးပါ။ Grid သို့မဟုတ် Guides ဖျောက်ထားလည်း Snapping ဖွင့်နေနိုင်သည်။ မမျှော်လင့်ဘဲ ငြိနေသေးလျှင် သက်ဆိုင်ရာ Snap command ကို ပိတ်ပါ။

Guides နှင့် Grid က နေရာချရန်သာဖြစ်ပြီး Export ပုံထဲ မပါပါ။ Guides ကို `.lime` Project ထဲ သိမ်းသည်။ Grid spacing နှင့် Snapping က Personal settings ဖြစ်သည်။ Guide အစား Canvas နှင့် တိုက်ရိုက်ညှိရန် **Layer → Align** သုံးပါ။

## Layer များ အတူရွှေ့ရန် {#links}

အတူရွှေ့လိုသော Layer တစ်ခုစီ၏ Chain toggle ကို ဖွင့်ပါ။ Movement Link ဖွင့်ထားသော Layer တစ်ခုကို ရွှေ့လျှင် အခြားဖွင့်ထားသော Layer များလည်း အတူရွှေ့မည်။ ဖျောက်ထားသော Layer များလည်း ပါသည်။ Scale နှင့် Rotate က Layer တစ်ခုချင်းစီကိုသာ သက်ရောက်သည်။ ပါဝင်သော Layer တစ်ခုခုတွင် Position Lock ရှိလျှင် အားလုံးရွှေ့မရပါ။

Drop Shadow နှင့် Stroke Layer များက Attachment ကြောင့် Source နောက်လိုက်ရွှေ့သည်။ Source Pixel သို့မဟုတ် Text ပြောင်းသော်လည်း ထို Effect ပုံကို ပြန်မလုပ်ပါ။ သီးခြားရွှေ့ရန် **Layer → Detach from Source** သုံးပါ။ Raster Layer ကို ချက်ချင်းအပေါ်ရှိ Layer နှင့် တွဲရန် **Attach to Layer Above** သုံးနိုင်သည်။ တွဲလိုက်ရုံဖြင့် Pixel နေရာမပြောင်းပါ။

## ပြဿနာအဖြေရှာရန် {#troubleshooting}

Import လုပ်ထားသောပုံ ကြီးလွန်းလျှင် Handle များ Workspace အပြင်ရောက်နေနိုင်သည်။ **Layer → Fit Layer to Canvas** က Transparent margin များအပါအဝင် Layer အပြည့်ကို Scale လုပ်ပြီး အလယ်ချသည်။ Scale Layer သုံးနေစဉ်ဆို Preview ကို ပြင်ပေးသည်။ အခြား Tools မှဆို ချက်ချင်း Apply လုပ်သည်။ Canvas ကို ချဲ့လိုလျှင် **Image → Canvas Size…** သုံးပါ။

## ဆက်ဖတ်ရန် {#related}

- [ပုံများကို Layer အဖြစ် Import လုပ်ရန်](../../image/import-layers/)
- [Layer Transform](../../reference/transforms/)
- [Layer Move သို့မဟုတ် Transform လုပ်မရခြင်း](../../troubleshooting/moving/)
