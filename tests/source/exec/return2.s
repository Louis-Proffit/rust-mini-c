	.text
	.globl main
f:
	pushq %rbp
	movq %rsp, %rbp
	addq $-24, %rsp
	movq %rdi, %rax
	movq %rsi, -8(%rbp)
	movq $2, -16(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r11
	imulq %r10, %r11
	movq %r11, -16(%rbp)
	addq -16(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	movq $65, %rdi
	movq $0, %rsi
	call f
	movq %rax, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rsi
	call f
	movq %rax, %rdi
	call putchar
	movq $65, %rdi
	movq $2, %rsi
	call f
	movq %rax, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data

