

int main()
  return func(5+func_(3)*2-func(5)+func_(1+ funca(2) - funca(11)+3))  + funcb(3) + accumulate(10)

int func(int a)
  if a < 2
    return a
  else 
    return func(a-2) + func(a-1)
  
int func_(int a)
  return a * a

int funca(int a)
  if a > 10
    return a 
  else
    if a > 5
      return a * a
    else 
      return a * a * a

int funcb(int a) 
  int b = 0
  while a > 0
    b = b + a
    a = a - 1
  return b

int accumulate(int a)
  int res = 0
  int cur = 1
  while cur < a
    if cur > a - 2
      res = res + 1
    else
      pass
    res = res + cur
    cur = cur + 1
  return res