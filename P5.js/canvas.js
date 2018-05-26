let snake = new Snake(10, 11, 10);
let pause = true;

function setup () {
    stroke(0)
    let cvs = createCanvas(500, 500);
    let pauseButton = createButton('TOOGLE')
    pauseButton.mouseClicked(() => pause = !pause);
}

function draw () {

    for (let i = 0; i < 500; i++) {
        for (let j = 0; j < 500; j++) {
            rect(i * 10,j * 10, 10, 10);
        }   
    }

    if (!pause){
        clear()
        
        snake.show();
        snake.move(-0.1, 0);    
    }

}