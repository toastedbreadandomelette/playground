class heap:
    """
    Generalize binary heap, arranges elements according to the 
    comparison operator, by default initializes max heap.
    """
    def __init__(self, comparison_operator):
        self.heap_array = []
        if comparison_operator is not None:
            self.comparison_operator = comparison_operator
        else:
            # default max heap
            self.comparison_operator = lambda root, child: root >= child

    def push(self, item):
        """
        Pushes an element `item` into the heap.
        """
        # subsituting references for high performance
        ha, co = self.heap_array, self.comparison_operator
        ha.append(item)
        last_index = len(ha) - 1
        # rearrange the added element such the parent-children condition should be satisfied.
        # so swap until the parent-child is not satisfied.
        while last_index and not(co(ha[(last_index - 1) // 2], ha[last_index])):
            ha[last_index], ha[(last_index - 1)//2] = ha[(last_index - 1)//2], ha[last_index]
            last_index = (last_index - 1) // 2

    def heapify(self):
        """
        Used after popping off element from the heap.
        Rearrangement is done when a last element in the heap is put at the 
        top of the heap.

        Swapping is done in this stage, until the heap stage is satisfied:
        - A child is selected that can be the parent of root and another sibling
        - Swap these elements.
        - Repeat this process till such child is not found, or the root element follows
        condition.
        """
        i = 0
        # substitution for better performance
        ha, co = self.heap_array, self.comparison_operator
        # Follow this condition till the last of heap.
        while 2*i + 1 < len(ha):
            left, right = 2 * i + 1, 2 * i + 2
            # if there is no right child.
            if right == len(ha):
                if not(co(ha[i], ha[left])):
                    ha[i], ha[left] = ha[left], ha[i]
                    i = left
                else:
                    break
            elif right < len(ha):
                # check whether 'left' is correct for both root and
                # sibling
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
        """
        Pop element from heap: rearranges all element according to the comparator 
        """
        if len(self.heap_array) == 0:
            return None
        self.heap_array[0], self.heap_array[-1] = self.heap_array[-1], self.heap_array[0]
        item_to_return = self.heap_array.pop()
        self.heapify()
        return item_to_return

    def top(self):
        """
        Returns the top of the heap array.
        returns none if heap array is empty
        """
        if self.heap_array:
            return self.heap_array[0]
        return None

    def __len__(self):
        """
        Length of heap array
        """
        return len(self.heap_array)

    def __str__(self):
        """
        Print heap
        """
        return '<heap> %s ' % str(self.heap_array)
