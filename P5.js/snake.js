class Snake {
    constructor() {
        this.x = 10;
        this.y = 10;
        this.tail = [];
        this.dir = [1, 0];
        this.tailDir = []
        this.currTail = 0;

        this.dead = false;
    }

    
    getTailDir (idx) {
        if (idx === 0) {
            let x = (this.tail[idx][0] - this.x) * -1;
            let y = (this.tail[idx][1] - this.y) * -1; 
            return[x, y]
        } else {
            let x = (this.tail[idx][0] - this.tail[idx - 1][0]) * -1;
            let y = (this.tail[idx][1] - this.tail[idx - 1][1]) * -1;
            return [x, y]
        }
    }
    
    changeDir(dir) {
        
        this.lastDir = this.dir;

        if (this.dir[0] === 1) {
            if (dir === "UP") {
                this.dir = [0, -1];
            } 
            if (dir === "DOWN") {
                this.dir = [0, 1];
            }
        }
        if (this.dir[0] == -1) {
            if (dir === "UP") {
                this.dir = [0, -1];
            } 
            if (dir === "DOWN") {
                this.dir = [0, 1];
            }
        }
        if (this.dir[1] == 1) {
            if (dir === "LEFT") {
                this.dir = [-1, 0];
            } 
            if (dir === "RIGHT") {
                this.dir = [1, 0];
            }
        }
        if (this.dir[1] == -1) {
            if (dir === "LEFT") {
                this.dir = [-1, 0];
            } 
            if (dir === "RIGHT") {
                this.dir = [1, 0];
            }
        }

        
        
    }

    move() {


       for (let i = this.tail.length - 1; i > 0; i--) {
           this.tail[i] = this.tail[i - 1];
       } 

        this.x = this.x + (this.dir[0] * 1);
        this.y = this.y + (this.dir[1] * 1);

        
       this.tail[0] = [this.x, this.y]
        
        for (let i = 1; i < this.tail.length - 1; i++) {
            if ((this.tail[i][0] === this.tail[0][0]
                && this.tail[i][1] === this.tail[0][1])
                || this.x > 49 || this.x < 0
                || this.y > 49 || this.y < 0
            )
                this.dead = true; 
        }

    }

    eat (foodPos) {
        if (this.x == foodPos[0] && this.y == foodPos[1]) {
            return true;
        }
        return false;
    }
    
    grow() {
        if (this.tail.length === 0) {
            this.tail.push([this.x -  this.dir[0], this.y -  this.dir[1]])
        } else {
            this.tail.push([this.tail[this.tail.length - 1][0] -  this.dir[0], this.tail[this.tail.length - 1][1] -  this.dir[1]])
        }
    }

    show() {
        push();
        
        noStroke();
        for (let i = 0; i < this.tail.length; i++) {
            fill(255 - (i * 5));
            rect(floor(this.tail[i][0]) * 10, floor(this.tail[i][1]) * 10, 10, 10);
        }
        pop();
    }
}
