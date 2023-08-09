from brownie import *
import math

def token_amount(token, amount:float):
    return int(amount * 10**token.decimals())

class UniswapMath:
    def get_exr(self, token_in, token_out, price) -> float:
        return 10**token_out.decimals() * price / 10**token_in.decimals()

    def get_swap_amount(self, token_in, token_out, amount, price):
        exr = self.get_exr(token_in, token_out, price)
        return int(amount*exr)

    def get_sqrtRatioX96(self, token_in, token_out, price):
        exr = self.get_exr(token_in, token_out, price)
        return self.exr_to_sqrtRatioX96(exr)

    def get_tick(self, token_in, token_out, price):
        exr = self.get_exr(token_in, token_out, price)
        return self.exr_to_tick(exr)

    def tick_to_sqrtRatioX96(self, tick):
        exr = self.tick_to_exr(tick)
        return self.exr_to_sqrtRatioX96(exr)

    def exr_to_tick(self, exr):
        return int(math.log(exr, 1.0001))

    def exr_to_sqrtRatioX96(self, exr):
        return int(math.sqrt(exr) * 2 ** 96)

    def pricing(self, amount, tick):
        return amount * self.tick_to_exr(tick)

    def tick_to_exr(self, tick:int):
        return 1.0001 ** tick

    def sqrtRatioX96_to_exr(self, sqrtRatioX96):
        return sqrtRatioX96 ** 2 / 2 ** 192

def tick_space_floor(pool, tick):
    tick_space = pool.tickSpacing()
    return tick // tick_space * tick_space