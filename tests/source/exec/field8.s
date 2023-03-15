	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -8(%rbp)
	addq $0, %rsp
	movq -8(%rbp), %rdi
	movq $65, %r8
	movq -8(%rbp), %rcx
	movq %rcx, 0(%r8)
	movq -8(%rbp), %rsi
	movq 0(%rsi), %rdi
	call putchar
	addq $0, %rsp
	movq $66, %rdi
	movq -8(%rbp), %rsi
	movq %rsi, 8(%rdi)
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

