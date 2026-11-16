---
title: "ပုံများကို Layers အဖြစ် Import လုပ်ရန်"
description: "Tab အသစ်မဖွင့်ဘဲ ရှိပြီးသား Document ထဲသို့ ပုံဖိုင် သို့မဟုတ် Clipboard ပုံကို Layer အသစ်အဖြစ် ထည့်ရန်။"
translationKey: "image/import-layers"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Document ထဲ ပုံထပ်ထည့်ရန် {#steps}

1. ပုံထည့်မည့် Document tab ကို ရွေးပါ။
2. Drag လုပ်နေခြင်းကို အဆုံးသတ်ပြီး Transform preview ကို Apply သို့မဟုတ် Cancel လုပ်ပါ။
3. **File → Open as Layers…** သို့မဟုတ် **Cmd/Ctrl+L** ကို သုံးပါ။
4. Import လုပ်မည့် ပုံဖိုင်များကို ရွေးပါ။ ရှိပြီးသား Canvas ၏ Center တွင် Layers အသစ်အဖြစ် ထည့်ပေးသည်။
5. **Layers** မှ ထည့်ထားသော Layer ကို ရွေးပါ။ **Move** ဖြင့် နေရာချပါ သို့မဟုတ် **Scale Layer** ဖြင့် အရွယ်ပြင်ပြီး Transform ကို Apply လုပ်ပါ။

**ရလဒ်:** ပုံအသစ်များက Document တစ်ခုတည်းထဲတွင် ပေါ်လာမည်။ Canvas dimensions မပြောင်းပါ။ Layer ကြီးလျှင် Canvas အပြင်ထွက်နေနိုင်သည်။ Import လုပ်ခြင်းက အလိုအလျောက် Fit လုပ်ပေးခြင်း မဟုတ်ပါ။

## Import ပုံကို Fit လုပ်ရန် သို့မဟုတ် ပိုဖော်ရန် {#fit}

အချိုးမပျက် Fit လုပ်ရန် **Layer → Fit Layer to Canvas** ကို သုံးပါ။ Transparent margins ပါ ထည့်တွက်သည်။ ပုံသေးလျှင် ချဲ့နိုင်သည်။ Scale Layer သုံးနေစဉ် Fit က Preview ကို ပြောင်းသဖြင့် နောက်တွင် **Apply** လုပ်ပါ။

မူလ Pixel အရွယ်ကို ထိန်းပြီး ပတ်လည်နေရာ တိုးဖော်လိုလျှင် **Image → Canvas Size…** ကို သုံးပါ။ Document တစ်ခုလုံး၏ Bounds ပြောင်းမည်။ **Layer → Layer to Canvas Size** က Layer ကို လက်ရှိ Canvas နေရာတွင် Crop သို့မဟုတ် Pad လုပ်သည်။

## Clipboard နှင့် PDF {#other-sources}

**Edit → Paste** သို့မဟုတ် **Cmd/Ctrl+V** က Clipboard ပုံကို Center ချထားသော Layer အဖြစ် ထည့်သည်။ **File → New from Clipboard** က Document အသစ် ဖန်တီးသည်။ Text ပြင်နေစဉ် Paste က Image layer မထည့်ဘဲ Text draft ထဲ စာထည့်သည်။

Open as Layers တွင် တစ်ကြိမ်လျှင် PDF တစ်ဖိုင် ထည့်နိုင်သည်။ Import dialog မှ Pages နှင့် Resolution ရွေးပါ။ PDF pages က ရှိပြီးသား Canvas ပေါ်တွင် Raster layers ဖြစ်လာသည်။ Canvas ကို မချဲ့ပါ။ ရွေးထားသော Pages အားလုံးကို Undo တစ်ဆင့်အဖြစ် ထည့်သည်။ Import မအောင်မြင်လျှင် သို့မဟုတ် Cancel လုပ်လျှင် တစ်ပိုင်းတစ်စ မထည့်ပါ။

## Import ပုံ မတွေ့လျှင် {#troubleshooting}

Layers မှ Visibility နှင့် Layer order ကို စစ်ပါ။ Import လုပ်ထားသော Layer ကို ရွေးပြီး **View → Toggle Layer Boundary** ဖြင့် Bounds ကို ကြည့်ပါ။ Layer ကြီးလွန်းလျှင် အများစုက Canvas အပြင်တွင် ရှိနိုင်သည်။ PDF အတွက် Page ရွေးထားမှု၊ Canvas size သတိပေးချက်နှင့် Import ခလုတ်ပိတ်ထားရသည့် အကြောင်းပြချက်ကို စစ်ပါ။

## ဆက်ဖတ်ရန် {#related}

- [ပုံများ ပေါင်းပြီး နေရာချရန်](../../guides/combine-images/)
- [Image သို့မဟုတ် Canvas အရွယ်ပြင်ရန်](../../image/resize/)
- [PDF Import နှင့် Export](../../image/pdf/)
