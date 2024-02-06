	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rax
	movq $65, %rsi
	movq -8(%rbp), %rdi
	movq %rsi, 0(%rdi)
	movq -8(%rbp), %rsi
	movq 0(%rsi), %rdi
	call putchar
	movq $66, %r8
	movq -8(%rbp), %rdx
	movq %r8, 8(%rdx)
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

