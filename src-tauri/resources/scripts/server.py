import uvicorn
from fastapi import FastAPI
from rapidocr import EngineType, LangDet, ModelType, OCRVersion, RapidOCR

app = FastAPI()

# --- 1. OCR 引擎初始化 ---
print("初始化 OCR 引擎...")
engine = RapidOCR(
    params={
        "Det.engine_type": EngineType.OPENVINO,  # 检测引擎类型
        "Det.lang_type": LangDet.CH,            # 检测语言类型
        "Det.ocr_version": OCRVersion.PPOCRV5,  # 检测模型版本
        "Det.model_type": ModelType.MOBILE,     # 检测模型类型

        "Cls.engine_type": EngineType.OPENVINO,  # 分类引擎类型

        "Rec.engine_type": EngineType.OPENVINO,  # 识别引擎类型
        "Rec.ocr_version": OCRVersion.PPOCRV5,  # 识别模型版本
        "Rec.model_type": ModelType.MOBILE,     # 识别模型类型
    },
)
print("OCR引擎初始成功", engine)


@app.get("/health")
def health():
    # 只要能访问到这里，说明 Uvicorn 启动了，RapidOCR 也加载完了
    return {"status": "ok"}


if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=14242)
