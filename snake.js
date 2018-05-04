class Snake {
    
    constructor(length, x, y) {
        this.len = length;
        this.head = [x, y];
        this.body = new Array(this.len);
        for (let i = 0; i < length; i++) {
            this.body[i] = [x + i + 1, y];       
        }

        console.log(this.body)
    }

    move(dirX, dirY) {


        this.head[0] += dirX;
        this.head[1] += dirY;

        for (let i = 0; i < this.len; i++) {
            this.body[i][0] += dirX;
            this.body[i][1] += dirY;
        }

        console.log(this.body);

    }

    show() {

        noStroke();

        push();
        fill(0);
        rect(this.head[0] * 10, this.head[1] * 10, 10, 10);
        pop();
        
        for (let i = 0; i < this.len; i++) {
            push();
            fill(((i + 1) * (240/this.len)));
            rect(this.body[i][0] * 10, this.body[i][1] * 10, 10, 10)
            pop();      
        }
    }

    // getX = () => this.head[0];
    // getY = () => this.head[1];

}