.section .bss

.section .text
.globl scan

scan:
  # 分配栈空间 32 字节
  movq $0, %rdi
  movq $32, %rsi
  movq $3, %rdx
  movq $34, %r10
  movq $-1, %r8
  movq $0, %r9
  movq $9, %rax                # 存放栈底指针
  syscall

  # 读取输入到 buffer
  movq %rax, %rsi   # 缓冲区地址
  movq $0, %rax        # 系统调用号 sys_read
  movq $0, %rdi        # 文件描述符 0 (stdin)
  movq $32, %rdx       # 要读取的字节数
  syscall              # 执行系统调用

  # 转换 buffer 中的字符串为整数
  # movq $buffer, %rsi   # RSI 指向缓冲区的起始
  xor %rax, %rax       # RAX 存放最终的整数值
convert_loop:
  movzbq (%rsi), %rdx  # 读取一个字符
  cmpq $'\n', %rdx     # 检查是否为换行符
  je finished          # 如果是换行符，完成转换
  subq $'0', %rdx      # 将ASCII字符转换为相应的数字
  imulq $10, %rax      # RAX *= 10
  addq %rdx, %rax      # RAX += RDX
  inc %rsi             # 移动到下一个字符
  jmp convert_loop     # 继续循环
finished:
  ret                  # 返回，结果存储在 RAX 中
