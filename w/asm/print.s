  .globl print #标记为全局，才能被其他模块访问
  
print:
  # 分配栈空间 32 字节用于存储整数的字节表示
  movq $0, %rdi
  movq $32, %rsi
  movq $3, %rdx
  movq $34, %r10
  movq $-1, %r8  
  movq $0, %r9
  movq $9, %rax                # 存放栈底指针
  syscall

  # 转换函数
  movq %rax, %rdi              # rdi 指向字符串存储的起始位置
  movl (%rbp), %eax            # 将要转换的整数
  movl %eax, %ecx              # 把整数复制到 ecx，用作除法的被除数
  addq $34, %rdi               # 移动指针到字符串的末尾（假设最多32位加上终止符和换行符）
  movb $0, (%rdi)              # 设置字符串终止符
  decq %rdi                    # 指针回退到终止符前的位置
  movb $'\n', (%rdi)           # 添加换行符
  decq %rdi                    # 指针回退到换行符前的位置
convert_loop:
  xorl %edx, %edx              # 清除 edx 以供 div 使用
  movl $10, %ebx               # 除数设置为10
  divl %ebx                    # eax /= 10, edx = eax % 10
  addb $'0', %dl               # 将余数转换为ASCII
  movb %dl, (%rdi)             # 存储字符
  decq %rdi                    # 移动字符串指针
  testl %eax, %eax             # 检查是否还有数字需要处理
  jnz convert_loop             # 如果不为0，继续循环
  # 现在 %rdi 指向字符串的第一个字符，可以进行打印
  # 调用字符串打印系统服务
  mov %rdi, %rsi          # 消息的地址
  mov $1, %rax            # 系统调用号 1 (sys_write)
  mov $1, %rdi            # 文件描述符 1 (stdout)
  # mov $len, %rdx        # 消息的长度
  mov $100, %rdx          # 消息的长度
  syscall                 # 调用系统服务
  ret
  