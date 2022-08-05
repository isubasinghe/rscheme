(define (counter n)
  (if (ge n 0) (fmt "On {}" n) (counter (- n 1))))

