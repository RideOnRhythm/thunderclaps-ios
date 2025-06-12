var contentTarget = document.getElementById("imagePaste");
var button = document.getElementById("classify");
var clear = document.getElementById("clear");
var imageData;

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
    let img = new Image();
    if (imageData == null) {
        return;
    }
    img.src = imageData;

    img.onload = async () => {
        try {
            const tensor = tf.browser.fromPixels(img)
                .toFloat()
                .resizeBilinear([299, 299])
                .div(tf.scalar(255.0))
                .expandDims();
            const batched = tensor.reshape([1, 299, 299, 3])

            let model = await tf.loadLayersModel("/model/model.json");
            const prediction = model.predict(batched);
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
            console.error(err);
        }
    };
}

clear.onclick = () => {
    contentTarget.innerHTML = 'Paste image here...';
    let results = document.getElementById("results");
    results.innerHTML = '';
}
