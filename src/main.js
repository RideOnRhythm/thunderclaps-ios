let contentTarget = document.getElementById("imagePaste");
let button = document.getElementById("classify");
let imageData;

contentTarget.onpaste = (event) => {
    const items = event.clipboardData?.items;
    if (!items) return;

    for (const item of items) {
        if (item.type.startsWith('image/')) {
            const file = item.getAsFile();
            if (file) {
                const reader = new FileReader();

                reader.onload = (e) => {
                    imageData = e.target.result;
                }

                reader.readAsDataURL(file);
            }
            break; // Stop after the first image
        }
    }
};

button.onclick = () => {
    const img = new Image();
    if (imageData == null) {
        return;
    }
    img.src = imageData;

    img.onload = async () => {
        try {
            const tensor = tf.browser.fromPixels(img)
                .resizeNearestNeighbor([299, 299])
                .toFloat()
                .expandDims();

            let model = await tf.loadLayersModel("/model/model.json");
            const prediction = model.predict(tensor);
            const data = await prediction.data();

            let array = Array.from(data);
            let string = `Drawing: ${(array[0] * 100).toFixed(2)}%
Hentai: ${(array[1] * 100).toFixed(2)}%
Neutral: ${(array[2] * 100).toFixed(2)}%
Porn: ${(array[3] * 100).toFixed(2)}%
Sexy: ${(array[4] * 100).toFixed(2)}%
`;
            let results = document.getElementById("results");
            results.textContent = string;
        } catch (err) {

        }
    };
}
