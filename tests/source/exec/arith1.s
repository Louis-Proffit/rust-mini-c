	.text
	.globl main
main:
	movq $100, %rdi
	movq $4, %rax
	addq %rax, %rdi
	call putchar
	movq $102, %rdi
	movq $1, %r9
	subq %r9, %rdi
	call putchar
	movq $100, %rdi
	movq $2, %rsi
	movq $4, %rcx
	imulq %rcx, %rsi
	addq %rsi, %rdi
	call putchar
	movq $216, %rax
	movq $2, %r9
	cqto
	idivq %r9
	movq %rax, %rdi
	call putchar
	movq $3, %rdi
	movq $37, %rax
	imulq %rax, %rdi
	call putchar
	movq $32, %rdi
	call putchar
	movq $118, %rdi
	movq $1, %rdx
	movq $2, %rcx
	subq %rcx, %rdx
	movq $0, %rcx
	subq %rdx, %rcx
	addq %rcx, %rdi
	call putchar
	movq $100, %rdi
	movq $122, %rax
	movq $11, %r8
	cqto
	idivq %r8
	addq %rax, %rdi
	call putchar
	movq $113, %rdi
	movq $1, %rdx
	movq $2, %rax
	cmpq %rax, %rdx
	setl %dl
	addq %rdx, %rdi
	call putchar
	movq $108, %rdi
	movq $2, %rsi
	movq $1, %rcx
	cmpq %rcx, %rsi
	setl %sil
	addq %rsi, %rdi
	call putchar
	movq $99, %rdi
	movq $2, %rsi
	movq $1, %r8
	movq $1, %r9
	addq %r9, %r8
	cmpq %r8, %rsi
	sete %sil
	addq %rsi, %rdi
	call putchar
	movq $10, %rdi
	movq $1, %rcx
	movq $2, %r8
	cmpq %r8, %rcx
	sete %cl
	addq %rcx, %rdi
	call putchar
	movq $0, %rax
	ret
	.data

