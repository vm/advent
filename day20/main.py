import itertools
import math

class InfiniteHouses(object):
    def __init__(self, minimum_presents):
        """
        Initializes class with minimum presents and an empty memoized table.

        :param minimum_presents (int): Number of presents looking for in a
            house.
        """
        self.minimum_presents = minimum_presents
        self.memoized_mods = {}

    @staticmethod
    def find_sorted_factors(n):
        """
        Finds the factors of an integer.

        :param n (int): Number to find factors of.
        :returns (list of int): Sorted factors.
        """

        factors = []
        # Only need to go up to ceiling of square root of n since all the
        # factors would have been found.
        for potential_factor in xrange(1, int(math.ceil(n ** .5))):
            if not n % potential_factor:
                factors.append(potential_factor)
                factors.append(n // potential_factor)
        return sorted(factors)

    def search(self):
        """
        Finds the first house that has the minimum required number of presents
        delivers to it. Since there are infinite houses on this street, it
        iterates infinitely until a house that matches said criteria is found.

        :returns (int): Number of the the first house with at least the
            mininumum required presents.
        """
        for house in itertools.count(start=1):
            all_factors = set()
            for i in reversed(xrange(1, house)):
                if not i in all_factors and not house % i:
                    all_factors.update(set(self.find_sorted_factors(house)))
            self.memoized_mods[house] = all_factors
            total = sum(factor * 10 for factor in all_factors)
            if total >= self.minimum_presents:
                return house


if __name__ == '__main__':
    print InfiniteHouses(29000000).search()

