	.text
	.globl main
main:
	movq $100, %rdi
	movq $4, %rcx
	addq %rcx, %rdi
	call putchar
	movq $102, %rdi
	movq $1, %rdx
	subq %rdx, %rdi
	call putchar
	movq $100, %rdi
	movq $2, %rax
	movq $4, %rdx
	imulq %rdx, %rax
	addq %rax, %rdi
	call putchar
	movq $216, %rax
	movq $2, %r8
	cqto
	idivq %r8
	movq %rax, %rdi
	call putchar
	movq $3, %rdi
	movq $37, %rsi
	imulq %rsi, %rdi
	call putchar
	movq $32, %rdi
	call putchar
	movq $118, %rdi
	movq $1, %rax
	movq $2, %r9
	subq %r9, %rax
	movq $0, %r8
	subq %rax, %r8
	addq %r8, %rdi
	call putchar
	movq $100, %rdi
	movq $122, %rax
	movq $11, %rcx
	cqto
	idivq %rcx
	addq %rax, %rdi
	call putchar
	movq $113, %rdi
	movq $1, %r9
	movq $2, %rsi
	cmpq %rsi, %r9
	setl %r9b
	addq %r9, %rdi
	call putchar
	movq $108, %rdi
	movq $2, %r8
	movq $1, %rax
	cmpq %rax, %r8
	setl %r8b
	addq %r8, %rdi
	call putchar
	movq $99, %rdi
	movq $2, %rax
	movq $1, %rcx
	movq $1, %r8
	addq %r8, %rcx
	cmpq %rcx, %rax
	sete %al
	addq %rax, %rdi
	call putchar
	movq $10, %rdi
	movq $1, %r9
	movq $2, %rsi
	cmpq %rsi, %r9
	sete %r9b
	addq %r9, %rdi
	call putchar
	movq $0, %rax
	ret
	.data

