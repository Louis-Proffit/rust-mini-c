	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -8(%rbp)
	movq -8(%rbp), %r8
	movq $65, %rdx
	movq -8(%rbp), %rdi
	movq %rdx, 0(%rdi)
	movq -8(%rbp), %rcx
	movq 0(%rcx), %rdi
	call putchar
	movq $66, %r9
	movq -8(%rbp), %rdx
	movq %r9, 8(%rdx)
	movq -8(%rbp), %rsi
	movq 8(%rsi), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

