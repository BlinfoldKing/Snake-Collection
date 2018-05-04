let snake = new Snake(10, 11, 10);
let pause = true;

function setup () {
    stroke(0)
    let cvs = createCanvas(500, 500);
    let pauseButton = createButton('TOOGLE')
    pauseButton.mouseClicked(() => pause = !pause);
}

function draw () {

    if (!pause){
        clear()
        snake.show();
        snake.move(-0.1, 0);    
    }

}