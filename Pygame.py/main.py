from enum import Enum
import pygame as pg 
import random
class Direction(Enum):
    UP = 1
    DOWN = 2
    RIGHT = 3
    LEFT = 4

class Snake:

    body = []
    dir = Direction.RIGHT

    def __init__(self):
        self.body.append([10, 20])
        self.grow()
        self.grow()

    def move(self):
        for i in range(len(self.body) - 1, 0, -1):
            self.body[i] = self.body[i - 1].copy()
        if self.dir == Direction.UP:
            self.body[0][1] -= 1
        elif self.dir == Direction.DOWN:
            self.body[0][1] += 1
        elif self.dir == Direction.RIGHT:
            self.body[0][0] += 1
        elif self.dir == Direction.LEFT:
            self.body[0][0] -= 1

    def grow(self):
        newBody = self.body[-1].copy()
        if self.dir == Direction.UP:
            newBody[1] -= 1
        elif self.dir == Direction.DOWN:
            newBody[1] += 1
        elif self.dir == Direction.RIGHT:
            newBody[0] += 1
        elif self.dir == Direction.LEFT:
            newBody[0] -= 1 
        self.body.append(newBody)
    
    def changeDir(self, dir):
        self.dir = dir

    def eat(self, food):
        return self.body[0][0] == food[0] and self.body[0][1] == food[1] 

BLACK = (  3,   3,   3)
WHITE = (255, 255, 255)
RED =   (255,   0,   33)
 

class Game:
    
    WIDTH = 500
    HEIGHT = 500
    running = True
    food = [0, 0]

    def __init__(self):
        random.seed(1)
        pg.init()
        self.screen = pg.display.set_mode([self.WIDTH, self.HEIGHT])        
        self.snake = Snake()
        self.running = True
        self.generateFood()

    def generateFood(self):
        self.food = random.sample(range(0, 50), 2)

    def isRunning(self):
        return self.running

    def Update(self):
        self.snake.move()
        if self.snake.eat(self.food):
            self.generateFood()
            self.snake.grow()
        pg.time.delay(100)
        pg.display.flip()
        
    def Draw(self):
        self.screen.fill(BLACK)       
        pg.draw.rect(self.screen, RED, [
            self.food[0] * 10, 
            self.food[1] * 10, 
            10, 10
            ])         
        for s in self.snake.body: 
            pg.draw.rect(self.screen, WHITE, [
                (s[0] * 10), 
                (s[1] * 10), 
                10, 10
                ])
    
    def getKey(self):
        for event in pg.event.get():
            if event.type == pg.QUIT:
                self.isRunning = False
            elif event.type == pg.KEYDOWN:
                if event.key == pg.K_UP and self.snake.dir != Direction.DOWN:
                    self.snake.changeDir(Direction.UP)
                elif event.key == pg.K_DOWN and self.snake.dir != Direction.UP:
                    self.snake.changeDir(Direction.DOWN)
                elif event.key == pg.K_LEFT and self.snake.dir != Direction.RIGHT:
                    self.snake.changeDir(Direction.LEFT)
                elif event.key == pg.K_RIGHT and self.snake.dir != Direction.LEFT:
                    self.snake.changeDir(Direction.RIGHT)

    

if __name__ == '__main__':
    game = Game()
    running = True
    while game.isRunning:
        game.Update()
        game.Draw()
        game.getKey()
        
