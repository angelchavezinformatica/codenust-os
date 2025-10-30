"""Calculator Process"""

import ast
import asyncio
import operator
from kernel.process_table import Process


class CalculatorProcess(Process):
    """Simple calculator process"""

    def __init__(self):
        super().__init__()
        self.running = True

    async def run(self):
        """Main loop (idle until work arrives)"""
        # En un futuro, podría leer de un message bus
        await asyncio.sleep(0.1)

    async def calculate(self, expression: str) -> float | str:
        """Evaluate safely a math expression"""

        try:
            result = self._safe_eval(expression)
            return result
        except ValueError as e:
            return f"Error: {e}"

    def _safe_eval(self, expr: str) -> float:
        """Safe math evaluation using AST"""

        # Define allowed operators
        operators = {
            ast.Add: operator.add,
            ast.Sub: operator.sub,
            ast.Mult: operator.mul,
            ast.Div: operator.truediv,
            ast.Pow: operator.pow,
            ast.Mod: operator.mod,
        }

        def eval_node(node):
            if isinstance(node, ast.BinOp):
                left = eval_node(node.left)
                right = eval_node(node.right)
                op = operators[type(node.op)]
                return op(left, right)

            if isinstance(node, ast.Num):
                return node.n

            if isinstance(node, ast.Expression):
                return eval_node(node.body)

            raise ValueError("Invalid expression")

        tree = ast.parse(expr, mode="eval")
        return eval_node(tree.body)
