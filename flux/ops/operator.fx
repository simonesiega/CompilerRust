// flux/ops/operator.fx

system class Operator {

    // Operatore di assegnamento ( = )
    operator assign[T](self: &mut T, value: T) -> void {
        // Default: copia semplice
        *self = value;
    }

    // Operatore di somma ( + )
    operator add[T](self: T, other: T) -> T {
        // Default: somma bit per bit
        return self + other;
    }

    // Operatore di uguaglianza ( == )
    operator equals[T](self: T, other: T) -> bool {
        // Default: confronto binario
        return self == other;
    }

    // Operatore di confronto ( <=> )
    operator compare[T](self: T, other: T) -> Ordering {
        // Default: confronto binario
        if self < other {
            return Ordering::Less;
        }
        if self > other {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }

    // Operatore di negazione logica ( ! )
    operator not(self: bool) -> bool {
        return !self;
    }

    // Operatore di negazione aritmetica ( - )
    operator negate(self: int) -> int {
        return -self;
    }
}
