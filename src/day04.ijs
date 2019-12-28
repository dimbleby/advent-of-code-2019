load 'numeric'
ascending=: [: *./ 2 <:/\ ]
digits=: "."0@":

NB. Part one.
run=: [: +./ 2 =/\ ]
valid=: [: (run *. ascending) digits
+/ valid"0 range 231832 767346

NB. Part two.
pair=: [: +./ 0 1 0 E. 0 , 0 ,~ 2 =/\ ]
ok=: [: (pair *. ascending) digits
+/ ok"0 range 231832 767346
