(define (counter n)
  (if (> n 0) (fmt "On {}" n) (counter (n-1))))

