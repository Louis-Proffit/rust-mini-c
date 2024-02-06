	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rcx
	movq $24, %rdi
	call malloc
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $16, %rdi
	call malloc
	movq -8(%rbp), %rsi
	movq %rax, 8(%rsi)
	movq $65, %rdi
	movq -8(%rbp), %r8
	movq %rdi, 0(%r8)
	movq $66, %rcx
	movq -8(%rbp), %rdx
	movq %rcx, 16(%rdx)
	movq $120, %r8
	movq -8(%rbp), %r9
	movq 8(%r9), %rdi
	movq %r8, 0(%rdi)
	movq $121, %rax
	movq -8(%rbp), %r8
	movq 8(%r8), %r9
	movq %rax, 8(%r9)
	movq -8(%rbp), %rcx
	movq 0(%rcx), %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rdi
	movq 0(%rdi), %rdi
	call putchar
	movq -8(%rbp), %rsi
	movq 8(%rsi), %r9
	movq 8(%r9), %rdi
	call putchar
	movq -8(%rbp), %r9
	movq 16(%r9), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $88, %r9
	movq -16(%rbp), %r8
	movq %r9, 0(%r8)
	movq $89, %r8
	movq -16(%rbp), %rdx
	movq %r8, 8(%rdx)
	movq -8(%rbp), %rax
	movq 0(%rax), %rdi
	call putchar
	movq -8(%rbp), %rax
	movq 8(%rax), %r8
	movq 0(%r8), %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq 8(%rdi), %r8
	movq 8(%r8), %rdi
	call putchar
	movq -8(%rbp), %rcx
	movq 16(%rcx), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq -16(%rbp), %rdi
	movq -8(%rbp), %rdx
	movq %rdi, 8(%rdx)
	movq -8(%rbp), %rsi
	movq 0(%rsi), %rdi
	call putchar
	movq -8(%rbp), %rsi
	movq 8(%rsi), %rdi
	movq 0(%rdi), %rdi
	call putchar
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rax
	movq 8(%rax), %rdi
	call putchar
	movq -8(%rbp), %rsi
	movq 16(%rsi), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

