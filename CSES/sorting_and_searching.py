def distinct_numbers(number_list: list):
    """
    Returns number of distinct integers: (modifies the array)
    >>> distinct_numbers([2, 3, 2, 2, 3])
    2
    >>> distinct_numbers([2, 3, 2, 2, 3, 4])
    3
    """
    number_list.sort()
    start, unique = number_list[0], 1
    for number in number_list:
        if start != number:
            unique, start = unique + 1, number
    return unique

def apartments(applicant_req: list, apartment_sizes: list, max_allowed: int) -> int:
    """
    To distribute the apartments to the applicants so that 
    as many applicants can get the apartments. The sizes can
    vary as per allowed value.
    >>> apartments([60, 45, 80, 60], [30, 60, 75], 5)
    2
    >>> apartments([60, 45, 80, 60, 75, 33], [30, 60, 75, 65, 40, 30, 20], 10)
    5
    """
    apartment_sizes.sort()
    applicant_req.sort()

    i, count = 0, 0
    for applicant in applicant_req:
        while i < len(apartment_sizes) and apartment_sizes[i] < applicant - max_allowed:
            i += 1
        if i == len(apartment_sizes):
            break
        if apartment_sizes[i] >= applicant - max_allowed and apartment_sizes[i] <= applicant + max_allowed:
            count += 1
            i += 1
    return count

def ferris_wheel(max_weight: int, weight: list) -> int:
    """
    With n children and max two childrens with max weight for each of them,

    >>> ferris_wheel(10, [7, 2, 3, 9])
    3
    >>> ferris_wheel(10, [10, 9, 7, 10, 9, 8, 5, 6, 6, 5])
    9
    """

    def ferris_wheel_two_pointer(max_weight: int, weight: list) -> int:
        """
        """
        weight.sort()
        i, j, count = 0, len(weight) - 1, 0
        while i < j:
            while j > i and max_weight < weight[i] + weight[j]:
                count, j = count + 1, j - 1
            if i == j:
                count += 1
                break
            while i < j and max_weight >= weight[i] + weight[j]:
                count, i, j = count + 1, i + 1, j - 1
            if i == j:
                count += 1
                break
        return count

    return ferris_wheel_two_pointer(max_weight, weight)

def concert_tickets(ticket_price: int, max_customer_price: int) -> int:
    """
    """
    def upper_bound(search_list: list, search: int) -> int:
        low, high = 0, len(search_list)
        while low + 1 < high:
            mid = (low + high) // 2
            if search_list[mid] <= search:
                low = mid
            else:
                high = mid  
        return low + 1

    ticket_price.sort()
    next_avail, i = [i for i in range(len(ticket_price) + 1)], 0
    ticket_purchased = [-1]*len(max_customer_price)
    for idx, ticket in enumerate(max_customer_price):
        index = i = upper_bound(ticket_price, ticket)
        while i != next_avail[i]:
            i = next_avail[i]
        while index != i:
            next_avail[index], index = i, next_avail[index]
        if i:
            next_avail[i] = i - 1
            ticket_purchased[idx] = ticket_price[i - 1]
    return ticket_purchased

def restaurant_customers(times: list) -> int:
    """
    Given distinct timestamps of customers arrival times, and 
    leaving time, find the maximum customers at a given time.
    """
    class heap:
        def __init__(self, comparison_operator):
            self.heap_array = []
            if comparison_operator is not None:
                self.comparison_operator = comparison_operator
            else:
                # Max heap
                self.comparison_operator = lambda root, child: root >= child

        def push(self, item):
            ha, co = self.heap_array, self.comparison_operator
            ha.append(item)
            last_index = len(ha) - 1
            while last_index > 0 and not(co(ha[(last_index - 1) // 2], ha[last_index])):
                ha[last_index], ha[(last_index - 1)//2] = ha[(last_index - 1)//2], ha[last_index]
                last_index = (last_index - 1) // 2

        def heapify(self):
            i = 0
            ha, co = self.heap_array, self.comparison_operator
            while 2*i + 1 < len(ha):
                left, right = 2 * i + 1, 2 * i + 2
                if right == len(ha):
                    if not(co(ha[i], ha[left])):
                        ha[i], ha[left] = ha[left], ha[i]
                        i = left
                    else:
                        break
                elif right < len(ha):
                    if co(ha[left], ha[right]):
                        if not(co(ha[i], ha[left])):
                            ha[i], ha[left] = ha[left], ha[i]
                            i = left
                        else:
                            break
                    elif not(co(ha[i], ha[right])):
                        ha[i], ha[right] = ha[right], ha[i]
                        i = right
                    else:
                        break
                else:
                    break

        def pop(self):
            if len(self.heap_array) == 0:
                return None
            self.heap_array[0], self.heap_array[-1] = self.heap_array[-1], self.heap_array[0]
            item_to_return = self.heap_array.pop()
            self.heapify()
            return item_to_return

        def top(self):
            if self.heap_array:
                return self.heap_array[0]
            return None

        def __len__(self):
            return len(self.heap_array)

        def __str__(self):
            return '<heap> %s ' % str(self.heap_array)

    times.sort(key=lambda x: x[0])
    time_heap, max_customer = heap(lambda a, b: a < b), 0
    for time in times:
        arrival_time, leaving_time = time
        if time_heap is not None:
            while time_heap.top() is not None and time_heap.top() < arrival_time:
                time_heap.pop()
        time_heap.push(leaving_time)
        max_customer = max(max_customer, len(time_heap))
    return max_customer

def movie_festival(movie_list: list) -> int:
    """
    There are n movies with starting time and ending time.
    This function calculates the maximum movies that can be
    seen during the festival.
    >>> movie_festival([(3, 4), (5, 6), (7, 8)])
    3
    >>> movie_festival([(404, 882), (690, 974), (201, 255), (800, 933), (525, 819), (457, 601), (461, 978), (832, 932), (699, 804), (795, 860)])
    4
    """

    def lower_bound(search_list: list, search: int, start = 0) -> int:
        low, high = start, len(search_list)
        while low + 1 < high:
            mid = (low + high) // 2
            if search_list[mid][0] <= search:
                low = mid
            else:
                high = mid
        if search_list[low][0] < search:
            return low + 1
        return low

    movie_list.sort(key=lambda x: x[0])
    max_movies = [0] * len(movie_list)
    max_movies[-1] = 1
    for x in range(len(movie_list) - 2, -1, -1):
        next, next_total_movies = lower_bound(movie_list, movie_list[x][1], x), 0
        if next < len(movie_list):
            next_total_movies = 1 + max_movies[next]
        max_movies[x] = max(max_movies[x + 1], next_total_movies)
    return max_movies[0]

def sum_of_two_values(number_list: list, sum: int) -> int:
    """
    Find two numbers in lists that adds to a target sum.
    """
    unique_set = {}
    for x in range(len(number_list)):
        if sum - number_list[x] in unique_set:
            return '%d %d' % (x + 1, unique_set[sum - number_list[x]])
        else:
            unique_set[number_list[x]] = x+1
    return 'IMPOSSIBLE'

def maximum_subarray_sum(number_list: list) -> int:
    """
    Get maximum array sum for integer array.
    Implements Kadane algorithm for faster computation
    """
    max_sum, sum = number_list[0], 0
    for x in number_list:
        sum += x
        max_sum = max(max_sum, sum)
        if sum <= 0:
            sum = 0
    return max_sum

def stick_lengths(stick_length: list) -> int:
    """
    """
    if len(stick_length) == 1:
        return 0
    stick_length.sort()
    cumulative_sum = [stick_length[0]]
    for x in range(1, len(stick_length)):
        cumulative_sum.append(cumulative_sum[-1] + stick_length[x])
    min_cost = (cumulative_sum[-1] - cumulative_sum[1]) * (stick_length[0] * (len(stick_length) - 1))
    calculate_left = lambda index: \
                abs(cumulative_sum[index - 1] - (stick_length[index] * index))

    calculate_right = lambda index: \
                abs((cumulative_sum[-1] - cumulative_sum[index]) - (stick_length[index] * (len(stick_length) - 1 - index)))
    for x in range(1, len(stick_length)):
        min_cost = min(min_cost, calculate_left(x) + calculate_right(x))
    return min_cost
    
def missing_coin_sum(number_list: list) -> int:
    """
    """
    number_list.sort()
    smallest_sum = 1
    for x in number_list:
        if x <= smallest_sum:
            smallest_sum += x
        else:
            break
    return smallest_sum

def collecting_numbers(number_list: list) -> int:
    """
    """
    list_with_idx = [(number, i) for i, number in enumerate(number_list)]
    list_with_idx.sort(key=lambda x: x[0])
    total_rounds, idx = 1, 0
    while idx < len(list_with_idx):
        while idx + 1 < len(list_with_idx) and list_with_idx[idx][1] < list_with_idx[idx + 1][1]:
            idx += 1
        if idx + 1 < len(list_with_idx):
            total_rounds += 1
        idx += 1
    return total_rounds

def playlist(nlist: list) -> int:
    """
    Given set of playlist with music ids, find longest subarray 
    of unique playlist.
    >>> playlist([1, 2, 1, 3, 2, 7, 4, 2])
    5
    """
    unique = set()
    start, max_size = 0, 0
    for idx, number in enumerate(nlist):
        if number in unique:
            while start < idx and number != nlist[start]:
                unique.discard(nlist[start])
                start += 1
            start += 1
            unique.add(number)
            max_size = max(max_size, len(unique))
        else:
            unique.add(number)
            max_size = max(max_size, len(unique))
    return max_size

def towers(nlist: list) -> int:
    """
    """
    def lower_bound(buckets: list, value: int) -> int:
        low, high = 0, len(buckets)
        while low + 1 < high:
            mid = (low + high) // 2
            if buckets[mid][-1] > value:
                high = mid
            else:
                low = mid
        if buckets[low][-1] <= value:
            return low + 1
        return low

    buckets = []
    for x in range(len(nlist)):
        if not buckets:
            buckets.append([nlist[x]])
        else:
            index = lower_bound(buckets, nlist[x])
            if index == len(buckets):
                buckets.append([nlist[x]])
            else:
                buckets[index].append(nlist[x])
    return len(buckets)

if __name__ == "__main__":
    from doctest import testmod
    testmod()
