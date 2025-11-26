class BufferFullException(BufferError):
    def __init__(self, message): pass


class BufferEmptyException(BufferError):
    def __init__(self, message): pass


class Node:
    def __init__(self):
        self.value = None
        self.next = None

    def __str__(self):
        return f"{self.value} -> {self.next.value}"

    def clear_value(self):
        self.value = None


class CircularBuffer:
    def __init__(self, capacity):
        self.head_reader = self.head_writter = self.create_buffer(capacity)

    def create_buffer(self, capacity):
        first_node = actual_node = Node()
        for _ in range(capacity-1):
            actual_node.next = actual_node = Node()
        actual_node.next = first_node
        return first_node

    def read(self):
        if self.head_reader.value is None:
            raise BufferEmptyException("Circular buffer is empty")
        to_return = self.head_reader.value
        self.head_reader.clear_value()
        self.head_reader = self.head_reader.next
        return to_return

    def write(self, data):
        if self.head_writter.value is not None:
            raise BufferFullException("Circular buffer is full")
        self.head_writter.value = data
        self.head_writter = self.head_writter.next

    def overwrite(self, data):
        if self.head_writter is self.head_reader:
            self.head_reader = self.head_reader.next
        self.head_writter.value = data
        self.head_writter = self.head_writter.next

    def clear(self):
        actual_node = self.head_reader
        actual_node.clear_value()
        while (actual_node := actual_node.next).value:
            actual_node.clear_value()
        self.head_reader = self.head_writter
