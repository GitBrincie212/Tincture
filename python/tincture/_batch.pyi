from tincture import Color


class ColorBatch:
    def __init__(self, colors: list[Color]) -> None:
        """
        A ColorBatch is a stream of colors which is used when a developer wants to operate on many colors simultaneously
        at lightning speed. As opposed to single-color operations where they execute the instruction immediately and return
        the result right away, a ColorBatch translates the instructions you give it into a batch instruction set and **ONLY**
        when provoked via a ``ColorBatch.operate()``, that is when they execute all the instructions and return the result,
        the execution of colors is done in parallel

        **Note:** For performance reasons, ColorBatch doesn't store the underlying PyObject but rather the
        non-atomic integer representation of the color. This means checking via ``a is b`` will **ALWAYS** return
        false

        :param colors: The colors to compose the batch
        """
        ...

    def operate(self) -> "ColorBatch":
        """
        Consumes all the instructions of the ColorBatch returning the desired result as a new color batch. Developers
        may use ``ColorBatch.operate_inplace()`` to modify the already existing color batch instead of allocating a new instance

        :return: The new color batch resulted from the batch instructions
        """
        ...

    def operate_inplace(self) -> "ColorBatch":
        """
        Consumes all the instructions of the ColorBatch, modifying the color batch instance **in-place** and returning it.
        Developers may use ``ColorBatch.operate()`` to create a new color batch instead of allocating a new instance

        :return: The self color batch that was modified **in-place** from the batch instructions
        """
        ...

    def add(self, colors: list["Color"]) -> "ColorBatch":
        """
        Creates an instruction for adding multiple colors to this specific batch. Returning
        the self ColorBatch instance (for continuous operations)

        :param colors: The colors to add to the current batch
        :return: Returns the self color batch that was called on the method
        """
        ...

    def sub(self, colors: list["Color"]) -> "ColorBatch":
        """
        Creates an instruction for subtracting multiple colors to this specific batch. Returning
        the self ColorBatch instance (for continuous operations)

        :param colors: The colors to add to the current batch
        :return: Returns the self color batch that was called on the method
        """
        ...

    def mul(self, colors: list["Color"]) -> "ColorBatch":
        """
        Creates an instruction for multiplying multiple colors to this specific batch. Returning
        the self ColorBatch instance (for continuous operations)

        :param colors: The colors to multiply to the current batch
        :return: Returns the self color batch that was called on the method
        """
        ...

    def add_scalar(self, scalars: list[float], include_transparency: bool = True) -> "ColorBatch":
        """
        Creates an instruction for adding multiple scalars to this specific batch. Returning
        the self ColorBatch instance (for continuous operations)

        :param scalars: The scalars to multiply with the current batch
        :param include_transparency: Whenever or not to operate on the alpha channel as well
        :return: Returns the self color batch that was called on the method
        """
        ...

    def sub_scalar(self, scalars: list[float], include_transparency: bool = True) -> "ColorBatch":
        """
        Creates an instruction for subtracting multiple scalars to this specific batch. Returning
        the self ColorBatch instance (for continuous operations)

        :param scalars: The scalars to multiply with the current batch
        :param include_transparency: Whenever or not to operate on the alpha channel as well
        :return: Returns the self color batch that was called on the method
        """
        ...

    def mul_scalar(self, scalars: list[float], include_transparency: bool = True) -> "ColorBatch":
        """
        Creates an instruction for multiplying multiple scalars to this specific batch. Returning
        the self ColorBatch instance (for continuous operations)

        :param scalars: The scalars to multiply with the current batch
        :param include_transparency: Whenever or not to operate on the alpha channel as well
        :return: Returns the self color batch that was called on the method
        """
        ...

    def div_scalar(self, scalars: list[float], include_transparency: bool = True) -> "ColorBatch":
        """
        Creates an instruction for dividing multiple scalars to this specific batch. Returning
        the self ColorBatch instance (for continuous operations)

        :param scalars: The scalars to multiply with the current batch
        :param include_transparency: Whenever or not to operate on the alpha channel as well
        :return: Returns the self color batch that was called on the method
        """
        ...

    def nth_root_scalar(self, scalars: list[float], include_transparency: bool = True) -> "ColorBatch":
        """
        Creates an instruction for taking the nth root of multiple scalars from this specific batch. Returning
        the self ColorBatch instance (for continuous operations)

        :param scalars: The scalars to take the nth root off the current batch
        :param include_transparency: Whenever or not to operate on the alpha channel as well
        :return: Returns the self color batch that was called on the method
        """
        ...