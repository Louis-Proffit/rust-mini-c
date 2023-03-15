	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rdi
L4791:
	movq $1, %rsi
	subq %rsi, -8(%rbp)
	movq -8(%rbp), %rdx
	movq $1, %rsi
	addq %rsi, %rdx
	testq %rdx, %rdx
	jz L4779
	movq $65, %rdi
	movq -8(%rbp), %r8
	addq %r8, %rdi
	call putchar
	addq $0, %rsp
	jmp L4791
L4779:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

