	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $65, -8(%rbp)
	movq -8(%rbp), %rsi
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $1, %rdx
	testq %rdx, %rdx
	jnz L58
	movq $67, %rdi
	movq %rdi, %rsi
	call putchar
	addq $0, %rsp
L42:
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L58:
	movq $66, %rdi
	movq %rdi, %rdx
	call putchar
	addq $0, %rsp
	jmp L42
	.data

