(define (counter n)
  (if (> n 0) (fmt "On {}" n) (counter (n-1))))

(define hello 
  (write "Hello World"))
)

(define (writeontrue val truth)
  (if (== #t truth) (write val)))

(main
  (counter 10)
  (counter 1)
)
