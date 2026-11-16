---
title: "Project Save နှင့် ပုံ Export"
description: "Editable artwork ကို .lime ဖြင့် Save လုပ်ပြီး PNG၊ JPEG၊ WebP သို့မဟုတ် PDF အဖြစ် Flattened copy ထုတ်ရန်။"
translationKey: "reference/files"
sourceRevision: "1.2.2-guide-1"
translatedRevision: "1.2.2-guide-1"
---

## Editable မူရင်း Save လုပ်ရန် {#project}

1. **File → Save Project As…** မှ **`.lime`** ဖိုင်ကို Save လုပ်ပါ။
2. ထို Project ကို နောက်ထပ်ပြင်လျှင် **File → Save** ကို သုံးပါ။
3. Flattened export မျှဝေသည့်အခါ မူရင်းကို ဆက်ထိန်းထားပါ။ Export ပုံကို ပြန်ဖွင့်လည်း မူလ Layer stack သို့မဟုတ် Text source ကို ပြန်မရနိုင်ပါ။

Lime project က Lossless pixels၊ Layers၊ Editable Text၊ Paths၊ Guides၊ Masks၊ Blend modes၊ Movement Links နှင့် ပံ့ပိုးထားသော Retained effects ကို သိမ်းသည်။ ကိုယ်ပိုင် Tool settings၊ Recent colors နှင့် Gradient presets က Project ထဲမပါဘဲ စက်၏ Preferences ထဲတွင် ရှိသည်။ `.imageedit` project အဟောင်းများကိုလည်း ဖတ်နိုင်သည်။

## သီးခြား မိတ္တူ Export လုပ်ရန် {#steps}

1. **File → Export…** (**Cmd/Ctrl+Shift+E**) ကို သုံးပါ။
2. Format ရွေးပါ။ JPEG အတွက် Quality ပြင်ပါ။ PDF အတွက် DPI နှင့် Physical page size ကို စစ်ပါ။
3. Destination picker သို့ ဆက်သွားပြီး သီးခြား ဖိုင်အမည်ရွေးပါ။
4. ဆက်ထိန်းထားလိုသော Unsaved edits ရှိလျှင် `.lime` မူရင်းကိုလည်း Save လုပ်ပါ။

**ရလဒ်:** Export က မြင်ရသော ပေါင်းစပ်ပုံကို Flattened copy အဖြစ် ထုတ်သည်။ Document ၏ Project destination သို့မဟုတ် Saved state ကို မပြောင်းပါ။ နောက်ဆုံး အောင်မြင်စွာ ထုတ်ထားသော Format နှင့် သက်ဆိုင်ရာ Options ကို App restart ပြီးလည်း မှတ်ထားသည်။ Cancel လုပ်ထားသော သို့မဟုတ် မအောင်မြင်သော Export က ထို Settings ကို မအစားထိုးပါ။

## Format ရွေးရန် {#formats}

| Format | သင့်တော်သော အသုံး | သိထားရန် |
| --- | --- | --- |
| **PNG** | Artwork၊ Screenshots၊ Transparent ပုံများ | Lossless pixels နှင့် Transparency ကို ထိန်းသည်။ |
| **JPEG** | Opaque ဓာတ်ပုံများ | Lossy quality setting ရှိသည်။ Transparency မရပါ။ |
| **WebP** | Transparency လိုသော Flattened ပုံ | Editable layers သို့မဟုတ် Text ကို မထိန်းပါ။ |
| **PDF** | Canvas တစ်ခုကို Print ထုတ်ရန် | အဖြူနောက်ခံပေါ်ရှိ Flattened RGB ပုံတစ်မျက်နှာ။ Searchable text သို့မဟုတ် Vector paths မပါပါ။ |

Raster **Save/Save As…** ကို Paths၊ Guides၊ Editable Text မပါသော Plain layer တစ်ခုတည်းတွင်သာ သုံးနိုင်သည်။ Editable structure ပါလျှင် `.lime` project နှင့် Export ကို တွဲသုံးပါ။ PDF က Export format ဖြစ်ပြီး Save format မဟုတ်ပါ။

## Save လုပ်မရလျှင် {#troubleshooting}

မြင်ရသော Crop၊ Gradient နှင့် Transform previews ကို အရင် Apply/Cancel လုပ်ပါ။ Save/Export က ပံ့ပိုးထားသော Inline numeric နှင့် Text edits ကို အရင် အဆုံးသတ်ပြီးမှ ဖိုင်ရေးသည်။ Text commit မအောင်မြင်လျှင် ကျန်နေသော Draft ကို ပြင်ပြီး ပြန်ကြိုးစားပါ။ Destination တွင် ရေးခွင့်ရှိကြောင်း စစ်ပြီး ပြထားသော Error ကို ဖတ်ပါ။

## ဆက်ဖတ်ရန် {#related}

- [PDF Import နှင့် Export](../../image/pdf/)
- [Preview၊ Apply နှင့် Cancel](../../basics/previews/)
- [File၊ Text နှင့် Recovery ပြဿနာများ](../../troubleshooting/files-text-recovery/)
