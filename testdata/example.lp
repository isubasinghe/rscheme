(define (counter n :requires[(>= n 0)])
  (if (> n 0) (fmt "On {}" n) (counter (- n 1))))

(define (timestwo a :requires[(>= n 0)] :ensures[(== (* a 2))])
  (* 2 a))

(define (main)
  (counter 10))

