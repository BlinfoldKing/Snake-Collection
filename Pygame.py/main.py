from enum import Enum
import pygame as pg 

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

    def Update(selft):
        pass

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
        print(self.body)

BLACK = (  3,   3,   3)
WHITE = (255, 255, 255)
RED =   (255,   0,   33)
 

class Game:
    
    WIDTH = 500
    HEIGHT = 500
    running = True

    def isRunning(self):
        return self.running

    def __init__(self):
        pg.init()
        self.screen = pg.display.set_mode([self.WIDTH, self.HEIGHT])        
        self.snake = Snake()
        self.screen.fill(BLACK)        
        self.running = True

    def Update(self):
        pg.display.flip()
        
    def Draw(self):
        for s in self.snake.body: 
            pg.draw.rect(self.screen, WHITE, [
                (s[0] * 10), 
                (s[1] * 10), 
                10, 10
                ])
    
    def getKey(self):
        for event in pg.event.get():
            if event.type == pg.QUIT:
                pg.quit()
                # sys.exit()
            # elif event.type == KEYDOWN:
            #     if event.key == K_U   # elif event.type == KEYDOWN:
            #     if event.key == K_UP:
            #         snake.point(UP)
            #     elif event.key == K_DOWN:
            #         snake.point(DOWN)
            #     elif event.key == K_LEFT:
            #         snake.point(LEFT)
            #     elif event.key == K_RIGHT:
            #         snake.point(RIGHT)P:
            #         snake.point(UP)
            #     elif event.key == K_DOWN:
            #         snake.point(DOWN)
            #     elif event.key == K_LEFT:
            #         snake.point(LEFT)
            #     elif event.key == K_RIGHT:
            #         snake.point(RIGHT)

    

if __name__ == '__main__':
    game = Game()
    
    running = True
    while game.isRunning:
        game.Update()
        game.Draw()
        game.getKey()
        
