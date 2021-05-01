const conectaMonitor = () => {
	return document.querySelector('canvas').getContext("2d")
}


const imprimirpixel = (monitor) => {
    const x = 10
    const y = 10
    const largura = 1 //Pixels
    const altura = 1 //Pixels
    const cor = "white"
    monitor.fillStyle = cor;
    monitor.fillRect(x, y, largura, altura);
}

const ligarComputador = () => {
	const monitor = conectaMonitor()

	console.log("XPTO")

	imprimirQuadrado(monitor)
	
}


