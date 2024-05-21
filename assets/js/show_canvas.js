function showCanvas(event, exampleId, projectId) {
    const arrowIcon = event.target;
    const canvasContainer = arrowIcon.closest('h3').nextElementSibling;

    if (canvasContainer.style.display === 'none' || canvasContainer.style.display === '') {
        canvasContainer.style.display = 'block';
        arrowIcon.parentElement.removeChild(arrowIcon);
        // arrowIcon.classList.add('fa-chevron-up');

        const newCanvas = document.createElement('canvas');
        newCanvas.id = exampleId;
        newCanvas.className = 'full-width-canvas';
        newCanvas.width = 640;
        newCanvas.height = 360;
        canvasContainer.appendChild(newCanvas);

        const script = document.createElement('script');
        script.type = 'module';
        script.innerHTML = `
            import init, { start_browser } from '/assets/${projectId}/${exampleId}.js';
            async function run() {
                await init();
                start_browser(document.getElementById("${exampleId}"));
            }
            run();
        `;
        canvasContainer.appendChild(script);
    }
}