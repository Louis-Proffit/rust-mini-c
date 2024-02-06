	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rsi
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %rdi
	movq $65, %rsi
	movq -16(%rbp), %rax
	movq %rsi, 0(%rax)
	movq -8(%rbp), %rdi
	movq 0(%rdi), %rdi
	call putchar
	movq -16(%rbp), %rcx
	movq 0(%rcx), %rdi
	call putchar
	movq $66, %rdi
	movq -16(%rbp), %rax
	movq %rdi, 8(%rax)
	movq -8(%rbp), %rsi
	movq 8(%rsi), %rdi
	call putchar
	movq -16(%rbp), %rdi
	movq 8(%rdi), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

