let pause;
let snake;
let food = []
let pauseButton

function reset() {
    pause = false;
    snake = new Snake();    
    snake.grow();
    snake.grow();
    snake.grow();
    frameRate(5)
    food = [floor(random(0, 49)), floor(random(0, 49))]
    console.log(food);
    
}

function setup () {
    stroke(0)
    let cvs = createCanvas(500, 500);
    reset();
    
}


function draw () {
    clear();
    background('#333')
    
    if (snake.eat(food)) {
        food = [floor(random(0, 49)), floor(random(0, 49))]
        snake.grow();    
    }

    if (!snake.dead) {
        
        if (!pause){
            snake.move();
        }
        push()
        fill('#ff0033')
        rect(food[0] * 10, food[1] * 10, 10, 10)
        pop()
        snake.show();
            
    } else 
        reset();


    if (pause) {
        push();
        fill(255);
        textSize(30)
        text("PAUSE", 200, 250);
        pop();
    }

}


function keyPressed() {
    if (keyCode === UP_ARROW || keyCode === 87 || keyCode === 119){
        console.log("UP")
        snake.changeDir("UP");
    }else if (keyCode === DOWN_ARROW || keyCode === 83 || keyCode === 115) {
        console.log("DOWN")
        snake.changeDir("DOWN");
    }else if (keyCode === RIGHT_ARROW || keyCode === 68 || keyCode === 100) {
        console.log("RIGHT")
        snake.changeDir("RIGHT");
    }else if (keyCode === LEFT_ARROW || keyCode === 65 || keyCode === 97) {
        console.log("LEFT")
        snake.changeDir("LEFT");
    } else if (keyCode === 32) {
        pause = !pause
    }
}