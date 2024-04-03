

int main()
  return func(10-func_(1)) 


int func(int a)
  if 1 < a
    return func(a-1) + func(a-2)
  else
    return a
  
int func_(int a)
  return a * a